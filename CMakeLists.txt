cmake_minimum_required(VERSION 3.16)

project(jam3 VERSION 1.0.0 LANGUAGES CXX)

set(SOURCES
    src/main.cc
    src/drawer.cc
    src/grid.cc
    src/value.cc
)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -g")
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(jam ${SOURCES})
