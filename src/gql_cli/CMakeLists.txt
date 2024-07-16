include("${CMAKE_CURRENT_SOURCE_DIR}/vendor/cli11.cmake")
include("${CMAKE_CURRENT_SOURCE_DIR}/vendor/httprequest.cmake")

file(GLOB_RECURSE GQL_CLI_SOURCES src/*.cpp src/*.c)
add_executable(
    gql
    src/main.cpp
    ${GQL_CLI_SOURCES}
)
add_dependencies(gql HTTPRequest RapidJSON)
target_link_libraries(gql
    PRIVATE CLI11::CLI11
    PRIVATE libgql
)
target_include_directories(
    gql
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src/
    PRIVATE SYSTEM ${HTTPREQUEST_INCLUDE_DIR}
    PRIVATE SYSTEM ${CLI11_INCLUDE_DIR}
    PRIVATE SYSTEM ${RAPIDJSON_INCLUDE_DIR}
)
