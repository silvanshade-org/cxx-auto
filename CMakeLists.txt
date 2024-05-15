# NOTE: run with `cmake -G Ninja -S . -B build && cmake --build build`

cmake_minimum_required(VERSION 3.22.1)

project(cxx-auto LANGUAGES CXX)

# export CMake configuration to compile_commands.json for IDE support
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(cxx-auto STATIC
  crates/cxx-auto/cxx/lib/cxx-auto.cc
)
target_include_directories(cxx-auto PUBLIC
  crates
  target/cxxbridge
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
    -Wno-c99-extensions
    -Wno-dollar-in-identifier-extension
  )
endif()
