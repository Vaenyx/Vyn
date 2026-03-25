cmake_minimum_required(VERSION 3.20)

project(Vyn)

set(CMAKE_CXX_STANDARD 20)

add_executable(vyn src/main.cpp)

target_compile_features(vyn PRIVATE cxx_std_23)
