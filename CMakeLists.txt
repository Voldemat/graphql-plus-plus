cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    gql
    VERSION 0.0.1
    DESCRIPTION "Short description"
    HOMEPAGE_URL "https://example.com/"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    gql_gql
    source/gql.cpp
)
add_library(gql::gql ALIAS gql_gql)

include(GenerateExportHeader)
generate_export_header(
    gql_gql
    BASE_NAME gql
    EXPORT_FILE_NAME export/gql/gql_export.hpp
    CUSTOM_CONTENT_FROM_VARIABLE pragma_suppress_c4251
)

if(NOT BUILD_SHARED_LIBS)
  target_compile_definitions(gql_gql PUBLIC GQL_STATIC_DEFINE)
endif()

set_target_properties(
    gql_gql PROPERTIES
    CXX_VISIBILITY_PRESET hidden
    VISIBILITY_INLINES_HIDDEN YES
    VERSION "${PROJECT_VERSION}"
    SOVERSION "${PROJECT_VERSION_MAJOR}"
    EXPORT_NAME gql
    OUTPUT_NAME gql
)

target_include_directories(
    gql_gql ${warning_guard}
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>"
)

target_include_directories(
    gql_gql SYSTEM
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_BINARY_DIR}/export>"
)

target_compile_features(gql_gql PUBLIC cxx_std_20)

find_package(fmt REQUIRED)
target_link_libraries(gql_gql PRIVATE fmt::fmt)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Developer mode ----

if(NOT gql_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of gql"
  )
endif()

include(cmake/dev-mode.cmake)
