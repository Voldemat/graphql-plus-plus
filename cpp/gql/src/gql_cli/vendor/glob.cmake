include(FetchContent)

FetchContent_Declare(
    glob
    GIT_REPOSITORY https://github.com/p-ranav/glob.git
    GIT_TAG master
)
FetchContent_MakeAvailable(glob)
set_target_properties(glob_tests glob_tests_single PROPERTIES EXCLUDE_FROM_ALL 1 EXCLUDE_FROM_DEFAULT_BUILD 1)
