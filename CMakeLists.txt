cmake_minimum_required(VERSION 3.25.1)
project(my_project)

# GoogleTest requires at least C++14
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

enable_testing()

set(main_target rayla)
set(lib_target raylib)
set(include_dirs include)
set(main_sources
        src/main.c
        src/copy.c
)

# build both the rayla executable and a mock library to test against
add_executable(${main_target} ${main_sources})
add_library(${lib_target} ${main_sources})

include_directories(${include_dirs})
target_include_directories(${lib_target} PUBLIC ${include_dirs})

add_subdirectory(test)
