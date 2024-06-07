include(ExternalProject)
# Download RapidJSON
ExternalProject_Add(
    httprequest
    PREFIX "vendor/httprequest"
    GIT_REPOSITORY "https://github.com/elnormous/HTTPRequest.git"
    GIT_TAG 0ff67dbb7516e25d4b2e7bd133853361a84c85d2
    TIMEOUT 10
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
    INSTALL_COMMAND ""
    UPDATE_COMMAND ""
)

ExternalProject_Get_Property(httprequest source_dir)
set(HTTPREQUEST_INCLUDE_DIR ${source_dir}/include)
