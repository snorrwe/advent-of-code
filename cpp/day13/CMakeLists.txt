cmake_minimum_required(VERSION 3.10)
set(PROJECT_NAME aoc_day13)
project(${PROJECT_NAME} LANGUAGES CXX)

add_executable(${PROJECT_NAME} src/main.cpp)
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD 17)
set_property(TARGET ${PROJECT_NAME} PROPERTY CXX_STANDARD_REQUIRED ON)
# target_compile_options(${PROJECT_NAME} PRIVATE -Wno-pragmas)

# Minomaly

add_subdirectory(lib/minomaly)
target_link_libraries(${PROJECT_NAME} minomaly)
target_include_directories(${PROJECT_NAME} PRIVATE ${MINO_INCLUDES})
COPY_SDL_DLLS(${PROJECT_NAME})
