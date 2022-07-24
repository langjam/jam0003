cmake_minimum_required(VERSION 3.16)

project(jam3 VERSION 1.0.0 LANGUAGES CXX)

set(SOURCES
    src/main.cc
    src/parsing/token.cc
    src/parsing/lexer.cc
    src/parsing/parser.cc
    # src/runtime/drawer.cc
    # src/runtime/terminal_drawer.cc
    # src/runtime/grid.cc
    src/ast/exprs/addexpr.cc
    src/ast/exprs/mulexpr.cc
    src/ast/exprs/numberexpr.cc
    src/ast/exprs/specialexpr.cc
    src/ast/exprs/variableexpr.cc
    src/ast/instructions/generateinstruction.cc
    src/runtime/values/value.cc
    src/runtime/values/numbervalue.cc
    src/runtime/values/repeatvalue.cc
    src/runtime/values/combinationvalue.cc
    src/runtime/values/specialvalue.cc
    src/runtime/interpreter.cc
    src/runtime/simulation.cc
    src/runtime/command.cc
)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -g")
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
include_directories(src/)

add_executable(jam ${SOURCES})

find_package(Curses REQUIRED)
include_directories(${CURSES_INCLUDE_DIRS})
target_link_libraries(jam ${CURSES_LIBRARIES})
