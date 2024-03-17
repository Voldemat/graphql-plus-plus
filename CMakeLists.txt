cmake_minimum_required(VERSION 3.28.1)
project(graphql++ VERSION 0.0.1)

include(CMakePrintHelpers)
include(GNUInstallDirs)
include(FetchContent)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_C_EXTENSIONS OFF)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(THREADS_PREFER_PTHREAD_FLAG ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)

FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)
FetchContent_MakeAvailable(googletest)

file(GLOB_RECURSE GQL_HEADERS src/*.hpp src/*.h )
file(GLOB_RECURSE GQL_SOURCES src/*.cpp src/*.c )
list(REMOVE_ITEM GQL_SOURCES src/main.cpp)
file(GLOB_RECURSE GQL_TESTS_SOURCES src/*.cc )
add_library(
    gql_lib
    ${GQL_HEADERS}
    ${GQL_SOURCES}
)
add_executable(
    gql
    src/main.cpp
)
target_link_libraries(gql gql_lib) 
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/src/
    ${CMAKE_CURRENT_SOURCE_DIR}/src/include/
)
enable_testing()
cmake_print_variables(GQL_TESTS_SOURCES)
add_executable(
    main_test
    ${GQL_TESTS_SOURCES}
)
target_link_libraries(
    main_test
    PRIVATE GTest::gtest_main
    PRIVATE gql_lib
)
include(GoogleTest)
gtest_discover_tests(main_test)
