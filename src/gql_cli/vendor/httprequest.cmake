include(FetchContent)
set(BUILD_TESTING OFF CACHE INTERNAL "")
FetchContent_Declare(
    httprequest
    GIT_REPOSITORY https://github.com/elnormous/HTTPRequest.git
    GIT_TAG 0ff67dbb7516e25d4b2e7bd133853361a84c85d2
    GIT_SUBMODULES ""
    BUILD_COMMAND ""
    INSTALL_COMMAND ""
    UPDATE_COMMAND ""
)
FetchContent_MakeAvailable(httprequest)
set(HTTPREQUEST_INCLUDE_DIR ${HTTPRequest_SOURCE_DIR}/include)
