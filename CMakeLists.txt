cmake_minimum_required(VERSION 3.28)

project(cxx-auto LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_STANDARD 26)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# CXX selects the compiler at configure time; C++ modules need the Ninja
# generator. Cargo must generate cxxbridge headers before CMake builds this
# consumer of those headers.
add_library(cxx-auto STATIC cxx/lib/cmake.cxx)
target_sources(cxx-auto PUBLIC FILE_SET CXX_MODULES FILES cxx/module/cxx_auto.cppm)
target_include_directories(cxx-auto PUBLIC
  "${CMAKE_CURRENT_SOURCE_DIR}/cxx/include"
  "${CMAKE_CURRENT_SOURCE_DIR}/target/cxxbridge"
)
target_compile_options(cxx-auto PUBLIC
  -fno-exceptions
  -fno-rtti
  -Werror
  -Wall
  -Wextra
  -pedantic
)

enable_testing()
add_executable(cxx-auto-probes cxx/tests/probes.cxx)
target_link_libraries(cxx-auto-probes PRIVATE cxx-auto)
add_test(NAME probes COMMAND cxx-auto-probes)
