cmake_minimum_required(VERSION 3.25.1)
project(rayla VERSION 0.0.1 DESCRIPTION "ray tracing library")

# GoogleTest requires at least C++14
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_CXX_FLAGS "-Wall -Wextra -Werror -m64 -std=c++17")
set(CMAKE_CXX_FLAGS_RELEASE "-O3")
set(CMAKE_CXX_FLAGS_DEBUG "-O0 -g")
set(CMAKE_CXX_FLAGS_SANITIZED "-O0 -fsanitize=address")

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)

set(CMAKE_C_FLAGS "-Wall -Wextra -Werror -m64 -std=c17")
set(CMAKE_C_FLAGS_RELEASE "-O3")
set(CMAKE_C_FLAGS_DEBUG "-O0 -g")
set(CMAKE_C_FLAGS_SANITIZED "-O0 -g -fsanitize=address")

enable_testing()

set(lib_target raylib)
set(include_dirs include)
set(main_sources
        src/copy.c
)

# build both the rayla executable and a mock library to test against
add_executable(${PROJECT_NAME} src/main.c ${main_sources})
include_directories(${include_dirs})

if (BUILD_LIB)
	add_library(${lib_target} SHARED ${main_sources})
	target_include_directories(${lib_target} PUBLIC ${include_dirs})
endif()

if (BUILD_TESTS)
	add_subdirectory(test)
endif()
