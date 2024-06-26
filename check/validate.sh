#!/bin/sh
gql --help &> /dev/null
GQL_CALL_EXIT_CODE="$?";
if [ "$GQL_CALL_EXIT_CODE" = "0" ]; then
    gql internal parse-dir --client-dir './client' --server-dir './server' > schema.json || \
        graphql-inspector validate './client/**/*.graphql' './server/**/*.graphql'
else
    graphql-inspector validate './client/**/*.graphql' './server/**/*.graphql'
fi
