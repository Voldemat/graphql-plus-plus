# Graphql++ parser

## Usage

### As library
CMakeLists.txt
```cmake
cmake_minimum_required(VERSION 3.28.1)
project(check VERSION 0.0.1 LANGUAGES C CXX)
mark_as_advanced(CMAKE_MAKE_PROGRAM)
include(GNUInstallDirs)
include(CMakePrintHelpers)
include(FetchContent)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_C_EXTENSIONS OFF)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
set(THREADS_PREFER_PTHREAD_FLAG ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
add_compile_options(-fexperimental-library)

file(GLOB_RECURSE SHEADERS src/*.hpp src/*.h)
file(GLOB_RECURSE SSOURCES src/*.cpp src/*.c)
FetchContent_Declare(
  gql_proj
  GIT_REPOSITORY https://github.com/Voldemat/graphql-plus-plus.git
  GIT_TAG 938f9b5
)
FetchContent_MakeAvailable(gql_proj)
add_executable(
    main
    ${SHEADERS}
    ${SSOURCES}
)

target_link_libraries(main
    PRIVATE gql_lib
)

include_directories(
    ${GQL_PROJ_INCLUDE_DIR}
)
```

src/main.cpp
```cpp
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/tokens_accumulators.hpp"

int main() {
    lexer::VectorTokensAccumulator accum;
    lexer::Lexer lexer("asdsad %", &accum);
    lexer.parse();
    return 0;
};
```
