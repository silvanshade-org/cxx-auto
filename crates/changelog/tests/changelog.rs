//! Exercise the release-note renderer across the exact PR-to-squash boundary.

#[cfg(test)]
mod tests
{
    use std::fs;
    use std::os::unix::fs::PermissionsExt as _;
    use std::path::Path;
    use std::process::Command;
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    /// Failure propagated from a real Git command or the release-note renderer.
    type Failure = Box<dyn core::error::Error + Send + Sync>;

    /// Arguments of an isolated fixture Git process.
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct GitArgs<'command>(&'command [&'command str]);

    /// The only event histories exercised by the renderer fixture.
    #[derive(Clone, Copy)]
    enum Event
    {
        /// Render the proposed squash commit instead of transient review
        /// commits.
        PullRequest,
        /// Render the landed Git commits directly.
        Push,
    }

    /// Run a Git fixture operation and surface its exact failure.
    ///
    /// # Specification
    /// - provides: stdout from a successful command in the isolated repository.
    /// - fails: includes stderr and status when Git rejects the operation.
    /// - panics: none.
    ///
    /// # Errors
    /// - Returns a launch or UTF-8 decoding failure, or the Git status and
    ///   stderr.
    ///
    /// # Adequacy
    /// - hypothesis: On a fresh local repository, L3 reaches valid Git
    ///   mutations needed for versioned history; failed Git commands are
    ///   outside this fixture.
    /// - witness: `tests::open_review_matches_landed_squash_without_transient_commits`
    fn git(
        repo: &Path,
        args: GitArgs<'_>,
    ) -> Result<String, Failure>
    {
        // Command::args needs the raw slice; no trait boundary accepts GitArgs.
        let result = Command::new("git")
            .args(args.0)
            .current_dir(repo)
            .output()?;
        if !result.status.success() {
            return Err(format!(
                "git {args:?} failed ({}): {}",
                result.status,
                String::from_utf8_lossy(&result.stderr)
            )
            .into());
        }
        Ok(String::from_utf8(result.stdout)?)
    }

    /// Run the real renderer for an Actions event against the Git fixture.
    ///
    /// # Specification
    /// - provides: the generated changelog bytes from the fixture repository.
    /// - fails: surfaces renderer status and stderr if it cannot render.
    /// - panics: none.
    ///
    /// # Errors
    /// - Returns a process failure with stderr or a changelog read failure.
    ///
    /// # Adequacy
    /// - hypothesis: On a tagged fixture, L2 compares bytes returned for a
    ///   review and its landed squash; direct CLI calls separately observe
    ///   failed-render effects.
    /// - witness: `tests::open_review_matches_landed_squash_without_transient_commits`
    fn render(
        repo: &Path,
        event: Event,
    ) -> Result<String, Failure>
    {
        let result = Command::new(env!("CARGO_BIN_EXE_cxx-auto-changelog"))
            .current_dir(repo)
            // Actions uses string event names; no trait accepts this enum.
            .env(
                "GITHUB_EVENT_NAME",
                match event {
                    | Event::PullRequest => "pull_request",
                    | Event::Push => "push",
                },
            )
            .env("GITHUB_EVENT_PATH", repo.join("event.json"))
            .output()?;
        if !result.status.success() {
            return Err(format!(
                "changelog render failed ({}): {}",
                result.status,
                String::from_utf8_lossy(&result.stderr)
            )
            .into());
        }
        Ok(fs::read_to_string(repo.join("CHANGELOG.md"))?)
    }

    #[test]
    fn open_review_matches_landed_squash_without_transient_commits() -> Result<(), Failure>
    {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
        let repo =
            std::env::temp_dir().join(format!("cxx-auto-changelog-{}-{nonce}", std::process::id()));
        fs::create_dir_all(&repo)?;
        fs::write(repo.join("cliff.toml"), include_str!("../../../cliff.toml"))?;
        git(&repo, GitArgs(&["init", "-q", "-b", "main"]))?;
        git(&repo, GitArgs(&["config", "user.name", "Release fixture"]))?;
        git(
            &repo,
            GitArgs(&["config", "user.email", "fixture@example.test"]),
        )?;

        fs::write(repo.join("README.md"), "initial")?;
        git(&repo, GitArgs(&["add", "README.md"]))?;
        git(&repo, GitArgs(&["commit", "-qm", "Initial import"]))?;
        git(&repo, GitArgs(&["tag", "v0.0.2"]))?;
        fs::write(repo.join("README.md"), "release")?;
        git(&repo, GitArgs(&["add", "README.md"]))?;
        git(
            &repo,
            GitArgs(&["commit", "-qm", "feat(api): Landed release"]),
        )?;
        git(&repo, GitArgs(&["tag", "v0.0.3"]))?;
        fs::write(repo.join("README.md"), "landed")?;
        git(&repo, GitArgs(&["add", "README.md"]))?;
        git(
            &repo,
            GitArgs(&["commit", "-qm", "fix(api): Actual main change"]),
        )?;
        let base = git(&repo, GitArgs(&["rev-parse", "HEAD"]))?;
        git(&repo, GitArgs(&["switch", "-qc", "review"]))?;
        fs::write(repo.join("README.md"), "transient")?;
        git(&repo, GitArgs(&["add", "README.md"]))?;
        git(
            &repo,
            GitArgs(&["commit", "-qm", "feat(api): Should not survive squash"]),
        )?;
        let review_head = git(&repo, GitArgs(&["rev-parse", "HEAD"]))?;
        fs::write(
            repo.join("event.json"),
            format!(
                "{{\"pull_request\":{{\"number\":19,\"title\":\"feat(api): Reviewed capability\",\"base\":{{\"sha\":\"{}\"}}}}}}",
                base.trim()
            ),
        )?;

        let review = render(&repo, Event::PullRequest)?;
        assert!(review.contains("## Unreleased\n"));
        assert!(review.contains("### Unreleased Features\n"));
        assert!(review.contains("- _(api)_ Reviewed capability (#19)\n"));
        assert!(review.contains("- _(api)_ Actual main change\n"));
        assert!(review.contains("### 0.0.3 Features\n"));
        assert!(review.contains("- _(api)_ Landed release\n"));
        assert!(review.contains("### 0.0.2 Legacy\n"));
        assert!(review.contains("- Initial import\n"));
        assert!(!review.contains("Should not survive squash"));
        let changelog = repo.join("CHANGELOG.md");
        let file = fs::File::options().write(true).open(&changelog)?;
        file.set_times(fs::FileTimes::new().set_modified(UNIX_EPOCH))?;
        render(&repo, Event::PullRequest)?;
        assert_eq!(fs::metadata(&changelog)?.modified()?, UNIX_EPOCH);
        let fake_bin = repo.join("fake-bin");
        fs::create_dir_all(&fake_bin)?;
        let gh = fake_bin.join("gh");
        fs::write(&gh, "#!/bin/sh\nexit 1\n")?;
        fs::set_permissions(&gh, fs::Permissions::from_mode(0o755))?;
        let fixture_path = format!("{}:{}", fake_bin.display(), std::env::var("PATH")?);
        let no_review = Command::new(env!("CARGO_BIN_EXE_cxx-auto-changelog"))
            .current_dir(&repo)
            .env_remove("GITHUB_EVENT_NAME")
            .env_remove("GITHUB_EVENT_PATH")
            .env_remove("GH_REPO")
            .env("PATH", &fixture_path)
            .output()?;
        assert!(!no_review.status.success());
        assert!(String::from_utf8_lossy(&no_review.stderr).contains("gh failed"));

        git(&repo, GitArgs(&["switch", "-q", "main"]))?;
        fs::write(repo.join("README.md"), "merged")?;
        git(&repo, GitArgs(&["add", "README.md"]))?;
        git(
            &repo,
            GitArgs(&["commit", "-qm", "feat(api): Reviewed capability (#19)"]),
        )?;
        let landed = render(&repo, Event::Push)?;
        assert_eq!(
            review, landed,
            "review output must match the squash history"
        );
        let config = repo.join("cliff.toml");
        let hidden = repo.join("cliff.off");
        fs::rename(&config, &hidden)?;
        fs::write(&config, "[changelog\n")?;
        let no_config = Command::new(env!("CARGO_BIN_EXE_cxx-auto-changelog"))
            .current_dir(&repo)
            .env("GITHUB_EVENT_NAME", "push")
            .output()?;
        assert!(!no_config.status.success());
        assert!(String::from_utf8_lossy(&no_config.stderr).contains("git-cliff failed"));
        assert_eq!(fs::read_to_string(&changelog)?, landed);
        fs::remove_file(&config)?;
        fs::rename(hidden, config)?;
        fs::write(
            repo.join("event.json"),
            format!(
                "{{\"pull_request\":{{\"number\":19,\"title\":\"feat(api): Reviewed capability\",\"base\":{{\"sha\":\"{}\"}}}}}}",
                review_head.trim()
            ),
        )?;
        let stale = Command::new(env!("CARGO_BIN_EXE_cxx-auto-changelog"))
            .current_dir(&repo)
            .env("GITHUB_EVENT_NAME", "pull_request")
            .env("GITHUB_EVENT_PATH", repo.join("event.json"))
            .output()?;
        assert!(!stale.status.success());
        assert!(
            String::from_utf8_lossy(&stale.stderr).contains(&format!(
                "pull request base {} is not an ancestor of HEAD",
                review_head.trim()
            )),
            "{}",
            String::from_utf8_lossy(&stale.stderr)
        );
        assert_eq!(fs::read_to_string(repo.join("CHANGELOG.md"))?, landed);

        git(
            &repo,
            GitArgs(&["remote", "add", "origin", &repo.to_string_lossy()]),
        )?;
        git(&repo, GitArgs(&["switch", "-qc", "later-review"]))?;
        fs::write(repo.join("README.md"), "later transient")?;
        git(&repo, GitArgs(&["add", "README.md"]))?;
        git(
            &repo,
            GitArgs(&[
                "commit",
                "-qm",
                "feat(api): Should not survive later squash",
            ]),
        )?;
        fs::write(
            &gh,
            format!(
                "#!/bin/sh\nprintf '%s\\n' '{{\"number\":20,\"title\":\"feat(api): Later review\",\"baseRefOid\":\"{}\",\"baseRefName\":\"main\"}}'\n",
                base.trim()
            ),
        )?;
        fs::set_permissions(&gh, fs::Permissions::from_mode(0o755))?;
        let local = Command::new(env!("CARGO_BIN_EXE_cxx-auto-changelog"))
            .current_dir(&repo)
            .env_remove("GITHUB_EVENT_NAME")
            .env_remove("GITHUB_EVENT_PATH")
            .env_remove("GH_REPO")
            .env("PATH", &fixture_path)
            .output()?;
        assert!(
            local.status.success(),
            "{}",
            String::from_utf8_lossy(&local.stderr)
        );
        let current = fs::read_to_string(&changelog)?;
        assert!(current.contains("- _(api)_ Reviewed capability (#19)\n"));
        assert!(current.contains("- _(api)_ Later review (#20)\n"));
        assert!(!current.contains("Should not survive later squash"));
        fs::remove_dir_all(repo)?;
        Ok(())
    }
}
