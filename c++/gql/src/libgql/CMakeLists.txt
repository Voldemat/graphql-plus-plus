include("${CMAKE_CURRENT_SOURCE_DIR}/vendor/magic_enum.cmake")
file(GLOB_RECURSE GQL_LIB_HEADERS src/*.hpp src/*.h)
file(GLOB_RECURSE GQL_LIB_SOURCES src/*.cpp src/*.c)

filter_items(GQL_LIB_HEADERS ".*__tests__.*")
filter_items(GQL_LIB_SOURCES ".*__tests__.*")
add_library(
    libgql
    ${GQL_LIB_SOURCES}
)
add_dependencies(libgql RapidJSON)
target_include_directories(
    libgql
    INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/src/
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src/
    PRIVATE SYSTEM ${RAPIDJSON_INCLUDE_DIR}
)
target_link_libraries(
    libgql PRIVATE magic_enum::magic_enum
)
get_target_property(LIBB_INCLUDES libgql INCLUDE_DIRECTORIES)
if (LIBGQL_BUILD_TESTS)
    include("${CMAKE_CURRENT_SOURCE_DIR}/CMakeTestLists.txt")
endif()
