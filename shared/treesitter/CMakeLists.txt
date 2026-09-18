cmake_minimum_required(VERSION 3.31.7)
project(treesitter-plugin VERSION 0.0.1 LANGUAGES C)

if(NOT "$ENV{TREESITTER_CLI}" STREQUAL "")
    set(TREESITTER_CLI "$ENV{TREESITTER_CLI}")
else()
    set(
        TREESITTER_CLI
        "${CMAKE_CURRENT_SOURCE_DIR}/node_modules/.bin/tree-sitter"
    )
endif()
set(PARSERS_DIR "${CMAKE_CURRENT_BINARY_DIR}/parsers")
function(add_treesitter_parser GRAMMAR_NAME)
    set(PARSER_DIR "${PARSERS_DIR}/${GRAMMAR_NAME}")
    file(MAKE_DIRECTORY ${PARSER_DIR})
    message(STATUS "Generating parser for ${GRAMMAR_NAME}...")
    execute_process(
        COMMAND "${TREESITTER_CLI}"
            generate grammar.${GRAMMAR_NAME}.js
            --output "${PARSER_DIR}"
        WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
        RESULT_VARIABLE CLI_RESULT
        ERROR_VARIABLE CLI_ERROR
    )
    if(NOT CLI_RESULT EQUAL 0)
        message(
            FATAL_ERROR
            "Failed to generate parser for ${GRAMMAR_NAME}:\n${CLI_ERROR}"
        )
    endif()

    set(TARGET_NAME gql_${GRAMMAR_NAME})
    add_library(${TARGET_NAME} SHARED "${PARSER_DIR}/parser.c")
    set_target_properties(${TARGET_NAME} PROPERTIES
        C_STANDARD 23
        C_EXTENSIONS OFF
        CXX_EXTENSIONS OFF
    )
    target_include_directories(
        ${TARGET_NAME}
        PUBLIC ${PARSER_DIR}
    )
endfunction()

add_treesitter_parser("server")
add_treesitter_parser("client")
