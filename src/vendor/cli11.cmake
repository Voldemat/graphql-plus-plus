include(FetchContent)
FetchContent_Declare(
    cli11_proj
    QUIET
    GIT_REPOSITORY https://github.com/CLIUtils/CLI11.git
    GIT_TAG v2.4.1
)

FetchContent_MakeAvailable(cli11_proj)
set(CLI11_INCLUDE_DIR ${cli11_proj_SOURCE_DIR}/include/)
cmake_print_variables(CLI11_INCLUDE_DIR)
