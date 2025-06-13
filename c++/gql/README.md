# Graphql++ parser

## Usage

### As library
CMakeLists.txt
```cmake
cmake_minimum_required(VERSION 3.28.1)
project(check2 VERSION 0.0.1 LANGUAGES C CXX)
include(GNUInstallDirs)
include(FetchContent)

set(CMAKE_CXX_STANDARD 23)

file(GLOB_RECURSE SHEADERS src/*.hpp src/*.h)
file(GLOB_RECURSE SSOURCES src/*.cpp src/*.c)
FetchContent_Declare(
  gql_proj
  GIT_REPOSITORY https://github.com/Voldemat/graphql-plus-plus.git
  GIT_TAG d375637
)
FetchContent_MakeAvailable(gql_proj)
add_executable(
    main
    ${SSOURCES}
)
target_link_libraries(main PRIVATE libgql)
include_directories(
    PRIVATE ${libgql_INCLUDE_DIRS}
    PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/src
)
```

src/main.cpp
```cpp
#include "libgql/lexer/lexer.hpp"
#include "libgql/lexer/tokens_accumulators.hpp"

int main() {
    lexer::VectorTokensAccumulator accum;
    lexer::Lexer lexer("asdsad", &accum);
    lexer.parse();
    return 0;
};
```
