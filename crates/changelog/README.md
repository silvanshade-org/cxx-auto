# Changelog renderer

`cxx-auto-changelog` is private workspace tooling. `mise run changelog:render` invokes it, and treefmt runs that task before checking the root `CHANGELOG.md`. It replaces review-only commits with the proposed squash subject, or uses landed Git history on `main`, then calls the pinned `git-cliff` and Markdown formatter. Failed generation leaves the tracked changelog unchanged. The product crate does not depend on this tool, so release-note rendering does not build the C++ bridge.
