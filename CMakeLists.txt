# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.28.0)

project(cxx-auto LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(cxx-auto STATIC)
target_include_directories(cxx-auto PUBLIC
  target/cxxbridge
)
target_sources(cxx-auto PUBLIC
  FILE_SET CXX_MODULES FILES
    crates/cxx-auto/cxx/cxx-auto/cxx-auto.cppm
    crates/cxx-auto/cxx/cxx-auto/common/std.cppm
    crates/cxx-auto/cxx/cxx-auto/common/third_party/rust.cppm
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
    -Wno-c99-extensions
    -Wno-dollar-in-identifier-extension
  )
endif()

add_library(cxx-auto-example STATIC)
target_include_directories(cxx-auto-example PUBLIC
  crates/cxx-auto/cxx
  target/cxxbridge
)
target_sources(cxx-auto-example PUBLIC
  crates/cxx-auto-example/cxx/foo.cpp
)
target_sources(cxx-auto-example PUBLIC
  FILE_SET CXX_MODULES FILES
    crates/cxx-auto-example/cxx/foo.cppm
)
target_link_libraries(cxx-auto-example PUBLIC cxx-auto)
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
    -Wno-c99-extensions
    -Wno-dollar-in-identifier-extension
  )
endif()
