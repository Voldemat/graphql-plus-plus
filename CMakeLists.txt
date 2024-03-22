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
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)
enable_testing()
include("${CMAKE_SOURCE_DIR}/src/vendor/rapidjson.cmake")
find_package(absl REQUIRED)

file(GLOB_RECURSE GQL_HEADERS src/*.hpp src/*.h )
file(GLOB_RECURSE GQL_SOURCES src/*.cpp src/*.c )
list(REMOVE_ITEM GQL_SOURCES src/main.cpp)
file(GLOB_RECURSE GQL_TESTS_SOURCES src/*.cc )
file(GLOB_RECURSE LEXER_TESTS src/src/__tests__/lexer/cases/*.json)
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
    ${CMAKE_CURRENT_SOURCE_DIR}/src/src/
    ${CMAKE_CURRENT_SOURCE_DIR}/src/include/
    SYSTEM ${RAPIDJSON_INCLUDE_DIR}
    SYSTEM ${ABSL_INCLUDE_DIR}
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
)
add_dependencies(main_test rapidjson)
include(GoogleTest)
gtest_discover_tests(main_test)
