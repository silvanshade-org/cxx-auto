cmake_minimum_required(VERSION 3.22.1)

project(cxx-auto LANGUAGES CXX)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# CXX selects the compiler at configure time. Cargo must generate cxxbridge headers
# before CMake builds this consumer of those headers.
add_library(cxx-auto STATIC cxx/lib/cmake.cxx)
target_include_directories(cxx-auto PUBLIC
  "${CMAKE_CURRENT_SOURCE_DIR}/cxx/include"
  "${CMAKE_CURRENT_SOURCE_DIR}/target/cxxbridge"
)
target_compile_options(cxx-auto PUBLIC
  -std=c++2c
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
