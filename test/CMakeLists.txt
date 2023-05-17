# atctually download gtest (latest commit on main branch)
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/refs/heads/main.zip
)
# For Windows: Prevent overriding the parent project's compiler/linker settings
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

set(test_target raytest)
set(test_sources
        test_tuple.cpp
)

# build the test executable, linking against gtest as well as our mock library
add_executable(
  ${test_target}
  ${test_sources}
)
target_link_libraries(
  ${test_target}
  GTest::gtest_main
  ${lib_target}
)

# don't add test manually, just find them in the executable
include(GoogleTest)
gtest_discover_tests(${test_target})
