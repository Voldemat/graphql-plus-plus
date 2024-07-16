include(FetchContent)
set(RAPIDJSON_BUILD_DOC OFF CACHE INTERNAL "")
set(RAPIDJSON_BUILD_EXAMPLES OFF CACHE INTERNAL "")
set(RAPIDJSON_BUILD_TESTS OFF CACHE INTERNAL "")
set(RAPIDJSON_BUILD_CXX20 ON CACHE INTERNAL "")
FetchContent_Declare(
    rapidjson
    GIT_REPOSITORY https://github.com/Tencent/rapidjson.git
    GIT_TAG ab1842a2dae061284c0a62dca1cc6d5e7e37e346
    GIT_SUBMODULES ""
    CMAKE_ARGS
        -DCMAKE_COMPILER_ARGS="-D RAPIDJSON_HAS_CXX11_RVALUE_REFS 1"
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND ""
    UPDATE_COMMAND ""
)
FetchContent_MakeAvailable(rapidjson)
set(RAPIDJSON_INCLUDE_DIR ${rapidjson_SOURCE_DIR}/include)