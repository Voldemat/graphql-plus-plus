include(ExternalProject)
set(ABSL_INSTALL_PATH ${CMAKE_CURRENT_BINARY_DIR}/vendor/absl/src/abslproject-install/)
set(ABSL_PATCH_PATH ${CMAKE_SOURCE_DIR}/src/vendor/absl.patch)
ExternalProject_Add(
    abslproject
    PREFIX "vendor/absl"
    GIT_REPOSITORY "https://github.com/abseil/abseil-cpp.git"
    GIT_TAG 854193071498f330b71083d7e06a7cd18e02a4cc
    TIMEOUT 10
    PATCH_COMMAND git apply -v ${ABSL_PATCH_PATH}
    CONFIGURE_COMMAND cmake -DABSL_PROPAGATE_CXX_STD=ON -DCMAKE_INSTALL_PREFIX=${ABSL_INSTALL_PATH} -S ../abslproject -B .
    BUILD_COMMAND cmake --build . --target install
    INSTALL_COMMAND ""
    UPDATE_COMMAND ""
)

ExternalProject_Get_Property(abslproject BINARY_DIR)
set(ABSL_INCLUDE_DIR ${ABSL_INSTALL_PATH}/include)
set(ABSL_LIBRARY_DIR ${ABSL_INSTALL_PATH}/lib)
add_library(
    absl 
    INTERFACE
)
set_target_properties(
    absl
    PROPERTIES 
    INTERFACE_INCLUDE_DIRECTORIES ${ABSL_INCLUDE_DIR}
)
target_link_directories(
    absl INTERFACE ${ABSL_LIBRARY_DIR}
)
