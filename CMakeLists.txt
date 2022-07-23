cmake_minimum_required(VERSION 3.16)

project(jam3 VERSION 1.0.0 LANGUAGES CXX)

set(SOURCES
    src/main.cc
    src/parsing/token.cc
    src/parsing/lexer.cc
    src/parsing/parser.cc
    src/runtime/drawer.cc
    src/runtime/terminal_drawer.cc
    src/runtime/grid.cc
    src/runtime/value.cc
    src/ast/interpreter.cc
)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -g")
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
include_directories(src/)

add_executable(jam ${SOURCES})
