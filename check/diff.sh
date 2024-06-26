#!/bin/sh
sleep 1000000
gql --help &> /dev/null
GQL_CALL_EXIT_CODE="$?";
if [ "$GQL_CALL_EXIT_CODE" = "0" ]; then
    gql internal diff parse --path-to-schema ./schema.json --url-to-api "$1" || \
        graphql-inspector diff './server/**/*.graphql' "$1"
else
    graphql-inspector diff './server/**/*.graphql' "$1"
fi
