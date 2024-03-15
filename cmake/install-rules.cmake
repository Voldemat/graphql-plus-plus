if(PROJECT_IS_TOP_LEVEL)
  set(
      CMAKE_INSTALL_INCLUDEDIR "include/gql-${PROJECT_VERSION}"
      CACHE STRING ""
  )
  set_property(CACHE CMAKE_INSTALL_INCLUDEDIR PROPERTY TYPE PATH)
endif()

include(CMakePackageConfigHelpers)
include(GNUInstallDirs)

# find_package(<package>) call for consumers to find this project
set(package gql)

install(
    DIRECTORY
    include/
    "${PROJECT_BINARY_DIR}/export/"
    DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
    COMPONENT gql_Development
)

install(
    TARGETS gql_gql
    EXPORT gqlTargets
    RUNTIME #
    COMPONENT gql_Runtime
    LIBRARY #
    COMPONENT gql_Runtime
    NAMELINK_COMPONENT gql_Development
    ARCHIVE #
    COMPONENT gql_Development
    INCLUDES #
    DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
)

write_basic_package_version_file(
    "${package}ConfigVersion.cmake"
    COMPATIBILITY SameMajorVersion
)

# Allow package maintainers to freely override the path for the configs
set(
    gql_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/${package}"
    CACHE STRING "CMake package config location relative to the install prefix"
)
set_property(CACHE gql_INSTALL_CMAKEDIR PROPERTY TYPE PATH)
mark_as_advanced(gql_INSTALL_CMAKEDIR)

install(
    FILES cmake/install-config.cmake
    DESTINATION "${gql_INSTALL_CMAKEDIR}"
    RENAME "${package}Config.cmake"
    COMPONENT gql_Development
)

install(
    FILES "${PROJECT_BINARY_DIR}/${package}ConfigVersion.cmake"
    DESTINATION "${gql_INSTALL_CMAKEDIR}"
    COMPONENT gql_Development
)

install(
    EXPORT gqlTargets
    NAMESPACE gql::
    DESTINATION "${gql_INSTALL_CMAKEDIR}"
    COMPONENT gql_Development
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
