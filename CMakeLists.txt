# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.28.0)

project(cxx-auto LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(cxx-auto STATIC)
target_sources(cxx-auto PUBLIC
  FILE_SET HEADERS
  BASE_DIRS
    ${PROJECT_SOURCE_DIR}/target/cxxbridge
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto/cxx
  FILES
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto/cxx/cxx-auto/macros.hpp
)
target_sources(cxx-auto PUBLIC
  FILE_SET CXX_MODULES
  BASE_DIRS
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto/cxx
  FILES
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto/cxx/cxx-auto/cxx-auto.cppm
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto/cxx/cxx-auto/common/std.cppm
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto/cxx/cxx-auto/common/third_party/rust.cppm
)
set_target_properties(cxx-auto PROPERTIES
  CXX_STANDARD 23
  COMPILE_WARNING_AS_ERROR ON
)
if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
  target_compile_options(cxx-auto PUBLIC
    -Wall
    -Wextra
    -Wpedantic
  )
endif()
if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
  target_compile_options(cxx-auto PUBLIC
    -fmodules
    -fretain-comments-from-system-headers # clangd will complain about mismatch otherwise: https://github.com/clangd/clangd/issues/1105
    -Wno-c99-extensions
    -Wno-dollar-in-identifier-extension
  )
endif()

add_library(cxx-auto-example STATIC)
target_sources(cxx-auto-example PUBLIC
  ${PROJECT_SOURCE_DIR}/crates/cxx-auto-example/cxx/foo.cpp
  ${PROJECT_SOURCE_DIR}/crates/cxx-auto-example/ext/foo.cpp
)
target_sources(cxx-auto-example PUBLIC
  FILE_SET CXX_MODULES
  BASE_DIRS
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto-example/cxx
  FILES
    ${PROJECT_SOURCE_DIR}/crates/cxx-auto-example/cxx/foo.cppm
)
target_link_libraries(cxx-auto-example PUBLIC
  cxx-auto
)
set_target_properties(cxx-auto-example PROPERTIES
  CXX_STANDARD 23
  COMPILE_WARNING_AS_ERROR ON
)
if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang" OR CMAKE_CXX_COMPILER_ID STREQUAL "GNU")
  target_compile_options(cxx-auto-example PUBLIC
    -Wall
    -Wextra
    -Wpedantic
  )
endif()
if(CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
  target_compile_options(cxx-auto-example PUBLIC
    -fmodules
    -fretain-comments-from-system-headers # clangd will complain about mismatch otherwise: https://github.com/clangd/clangd/issues/1105
    -Wno-c99-extensions
    -Wno-dollar-in-identifier-extension
  )
endif()
