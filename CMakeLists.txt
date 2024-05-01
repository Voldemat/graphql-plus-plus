cmake_minimum_required(VERSION 3.28.1)
project(graphql++ VERSION 0.0.1)
mark_as_advanced(CMAKE_MAKE_PROGRAM)
include(GNUInstallDirs)
include(CMakePrintHelpers)
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
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)
enable_testing()
include("${CMAKE_SOURCE_DIR}/src/vendor/rapidjson.cmake")
include("${CMAKE_SOURCE_DIR}/src/vendor/absl.cmake")
include("${CMAKE_SOURCE_DIR}/src/vendor/cli11.cmake")

file(GLOB_RECURSE GQL_LIB_HEADERS src/src/libgql/*.hpp src/src/libgql/*.h)
file(GLOB_RECURSE GQL_LIB_SOURCES src/src/libgql/*.cpp src/src/libgql/*.c)
file(GLOB_RECURSE GQL_CLI_HEADERS src/src/gql_cli/*.hpp src/src/gql_cli/*.h)
file(GLOB_RECURSE GQL_CLI_SOURCES src/src/gql_cli/*.cpp src/src/gql_cli/*.c)
file(GLOB_RECURSE GQL_TESTS_SOURCES src/src/__tests__/*.cc)
file(GLOB_RECURSE GQL_TESTS_HEADERS src/src/__tests__/*.cc src/src/__tests__/*.hpp)
file(GLOB_RECURSE LEXER_TESTS src/src/__tests__/lexer/cases/*.json)
add_library(
    gql_lib
    ${GQL_LIB_HEADERS}
    ${GQL_LIB_SOURCES}
)
add_executable(
    gql
    src/main.cpp
    ${GQL_CLI_HEADERS}
    ${GQL_CLI_SOURCES}
)
target_link_libraries(gql
    PRIVATE CLI11::CLI11
    PRIVATE gql_lib
)
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/src/
    ${CMAKE_CURRENT_SOURCE_DIR}/src/src/
    ${CMAKE_CURRENT_SOURCE_DIR}/src/include/
    SYSTEM ${RAPIDJSON_INCLUDE_DIR}
    SYSTEM ${ABSL_INCLUDE_DIR}
    SYSTEM ${CLI11_INCLUDE_DIR}
)
add_custom_command(
    OUTPUT ${CMAKE_CURRENT_SOURCE_DIR}/src/src/__tests__/lexer/cases-lock.ls
    COMMAND ls ${CMAKE_CURRENT_SOURCE_DIR}/src/src/__tests__/lexer/cases/ > ${CMAKE_CURRENT_SOURCE_DIR}/src/src/__tests__/lexer/cases-lock.ls
    DEPENDS ${LEXER_TESTS}
)
set_source_files_properties(${CMAKE_CURRENT_SOURCE_DIR}/src/src/__tests__/lexer/cases-lock.ls PROPERTIES
    GENERATED TRUE
)
add_executable(
    main_test
    ${GQL_TESTS_SOURCES}
    ${CMAKE_CURRENT_SOURCE_DIR}/src/src/__tests__/lexer/cases-lock.ls
)
target_link_libraries(
    main_test
    PRIVATE GTest::gtest_main
    PRIVATE GTest::gmock_main
    PRIVATE gql_lib
    PRIVATE absl
)
add_dependencies(main_test rapidjson)
include(GoogleTest)
gtest_discover_tests(main_test)
