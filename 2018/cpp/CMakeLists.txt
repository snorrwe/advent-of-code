cmake_minimum_required(VERSION 3.10)

project(aoc LANGUAGES CXX)

add_subdirectory(lib/minomaly)

# Day 10

add_executable(aoc_day10 day10/src/main.cpp)
set_property(TARGET aoc_day10 PROPERTY CXX_STANDARD 17)
set_property(TARGET aoc_day10 PROPERTY CXX_STANDARD_REQUIRED ON)
target_link_libraries(aoc_day10 minomaly)
target_include_directories(aoc_day10 PRIVATE ${MINO_INCLUDES})
COPY_SDL_DLLS(aoc_day10)

# Day 15

add_executable(aoc_day15 day15/src/main.cpp)
set_property(TARGET aoc_day15 PROPERTY CXX_STANDARD 17)
set_property(TARGET aoc_day15 PROPERTY CXX_STANDARD_REQUIRED ON)
target_link_libraries(aoc_day15 minomaly)
target_include_directories(aoc_day15 PRIVATE ${MINO_INCLUDES})
COPY_SDL_DLLS(aoc_day15)
