if(PROJECT_IS_TOP_LEVEL)
  set(
      CMAKE_INSTALL_INCLUDEDIR "include/graphql---${PROJECT_VERSION}"
      CACHE STRING ""
  )
  set_property(CACHE CMAKE_INSTALL_INCLUDEDIR PROPERTY TYPE PATH)
endif()

include(CMakePackageConfigHelpers)
include(GNUInstallDirs)

# find_package(<package>) call for consumers to find this project
set(package graphql--)

install(
    DIRECTORY
    include/
    "${PROJECT_BINARY_DIR}/export/"
    DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
    COMPONENT graphql--_Development
)

install(
    TARGETS graphql--_graphql--
    EXPORT graphql--Targets
    RUNTIME #
    COMPONENT graphql--_Runtime
    LIBRARY #
    COMPONENT graphql--_Runtime
    NAMELINK_COMPONENT graphql--_Development
    ARCHIVE #
    COMPONENT graphql--_Development
    INCLUDES #
    DESTINATION "${CMAKE_INSTALL_INCLUDEDIR}"
)

write_basic_package_version_file(
    "${package}ConfigVersion.cmake"
    COMPATIBILITY SameMajorVersion
)

# Allow package maintainers to freely override the path for the configs
set(
    graphql--_INSTALL_CMAKEDIR "${CMAKE_INSTALL_LIBDIR}/cmake/${package}"
    CACHE STRING "CMake package config location relative to the install prefix"
)
set_property(CACHE graphql--_INSTALL_CMAKEDIR PROPERTY TYPE PATH)
mark_as_advanced(graphql--_INSTALL_CMAKEDIR)

install(
    FILES cmake/install-config.cmake
    DESTINATION "${graphql--_INSTALL_CMAKEDIR}"
    RENAME "${package}Config.cmake"
    COMPONENT graphql--_Development
)

install(
    FILES "${PROJECT_BINARY_DIR}/${package}ConfigVersion.cmake"
    DESTINATION "${graphql--_INSTALL_CMAKEDIR}"
    COMPONENT graphql--_Development
)

install(
    EXPORT graphql--Targets
    NAMESPACE graphql--::
    DESTINATION "${graphql--_INSTALL_CMAKEDIR}"
    COMPONENT graphql--_Development
)

if(PROJECT_IS_TOP_LEVEL)
  include(CPack)
endif()
