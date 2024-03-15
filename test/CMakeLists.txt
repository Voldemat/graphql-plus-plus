cmake_minimum_required(VERSION 3.14)

project(gqlTests LANGUAGES CXX)

include(../cmake/project-is-top-level.cmake)
include(../cmake/folders.cmake)

# ---- Dependencies ----

if(PROJECT_IS_TOP_LEVEL)
  find_package(gql REQUIRED)
  enable_testing()
endif()

find_package(Catch2 REQUIRED)
include(Catch)

# ---- Tests ----

add_executable(gql_test source/gql_test.cpp)
target_link_libraries(
    gql_test PRIVATE
    gql::gql
    Catch2::Catch2WithMain
)
target_compile_features(gql_test PRIVATE cxx_std_20)

catch_discover_tests(gql_test)

# ---- End-of-file commands ----

add_folders(Test)
