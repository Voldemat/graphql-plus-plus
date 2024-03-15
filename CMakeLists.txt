cmake_minimum_required(VERSION 3.14)

include(cmake/prelude.cmake)

project(
    graphql--
    VERSION 0.0.1
    DESCRIPTION "Graphql++"
    HOMEPAGE_URL "https://example.com/"
    LANGUAGES CXX
)

include(cmake/project-is-top-level.cmake)
include(cmake/variables.cmake)

# ---- Declare library ----

add_library(
    graphql--_graphql--
    source/graphql--.cpp
)
add_library(graphql--::graphql-- ALIAS graphql--_graphql--)

include(GenerateExportHeader)
generate_export_header(
    graphql--_graphql--
    BASE_NAME graphql--
    EXPORT_FILE_NAME export/graphql--/graphql--_export.hpp
    CUSTOM_CONTENT_FROM_VARIABLE pragma_suppress_c4251
)

if(NOT BUILD_SHARED_LIBS)
  target_compile_definitions(graphql--_graphql-- PUBLIC GRAPHQL___STATIC_DEFINE)
endif()

set_target_properties(
    graphql--_graphql-- PROPERTIES
    CXX_VISIBILITY_PRESET hidden
    VISIBILITY_INLINES_HIDDEN YES
    VERSION "${PROJECT_VERSION}"
    SOVERSION "${PROJECT_VERSION_MAJOR}"
    EXPORT_NAME graphql--
    OUTPUT_NAME graphql--
)

target_include_directories(
    graphql--_graphql-- ${warning_guard}
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>"
)

target_include_directories(
    graphql--_graphql-- SYSTEM
    PUBLIC
    "\$<BUILD_INTERFACE:${PROJECT_BINARY_DIR}/export>"
)

target_compile_features(graphql--_graphql-- PUBLIC cxx_std_20)

find_package(fmt REQUIRED)
target_link_libraries(graphql--_graphql-- PRIVATE fmt::fmt)

# ---- Install rules ----

if(NOT CMAKE_SKIP_INSTALL_RULES)
  include(cmake/install-rules.cmake)
endif()

# ---- Examples ----

if(PROJECT_IS_TOP_LEVEL)
  option(BUILD_EXAMPLES "Build examples tree." "${graphql--_DEVELOPER_MODE}")
  if(BUILD_EXAMPLES)
    add_subdirectory(example)
  endif()
endif()

# ---- Developer mode ----

if(NOT graphql--_DEVELOPER_MODE)
  return()
elseif(NOT PROJECT_IS_TOP_LEVEL)
  message(
      AUTHOR_WARNING
      "Developer mode is intended for developers of graphql--"
  )
endif()

include(cmake/dev-mode.cmake)
