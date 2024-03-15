cmake_minimum_required(VERSION 3.14)

project(graphql--Tests LANGUAGES CXX)

include(../cmake/project-is-top-level.cmake)
include(../cmake/folders.cmake)

# ---- Dependencies ----

if(PROJECT_IS_TOP_LEVEL)
  find_package(graphql-- REQUIRED)
  enable_testing()
endif()

find_package(Catch2 REQUIRED)
include(Catch)

# ---- Tests ----

add_executable(graphql--_test source/graphql--_test.cpp)
target_link_libraries(
    graphql--_test PRIVATE
    graphql--::graphql--
    Catch2::Catch2WithMain
)
target_compile_features(graphql--_test PRIVATE cxx_std_20)

catch_discover_tests(graphql--_test)

# ---- End-of-file commands ----

add_folders(Test)
