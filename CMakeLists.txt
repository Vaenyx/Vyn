include(FetchContent)
cmake_minimum_required(VERSION 4.3)
set(CMAKE_CXX_STANDARD 20)

project(Vyn)

find_package(CLI11 QUIET)

if (NOT CLI11_FOUND)
  message(STATUS "CLI11 not found, fetching...")
  FetchContent_Declare(
    CLI11
    GIT_REPOSITORY https://github.com/CLIUtils/CLI11.git
    GIT_TAG v2.4.2
  )
  FetchContent_MakeAvailable(CLI11)
endif()

add_executable(vyn src/main.cpp)

target_link_libraries(vyn PRIVATE CLI11::CLI11)
target_compile_features(vyn PRIVATE cxx_std_23)
