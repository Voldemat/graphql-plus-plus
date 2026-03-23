/* eslint-disable max-lines */
import { loadESLint } from "eslint";
import { join } from "path";
import {
    type ActorContext,
    actors,
    type Config,
    run
} from '@vladimirdev635/gql-codegen'
import {
    buildClientSchemaFromServerSchema,
    loadServerSchemaFromFile,
} from '@vladimirdev635/gql-codegen/schema/utils'

const baseTsConfig: actors.ts.TSActorConfig = {
    tsconfigCompilerOptions: actors.ts.loadTsConfigCompilerOptions(),
    formatters: [
        await actors.ts.formatters.eslint.buildESLintFormatter(loadESLint)
    ]
}
const server = loadServerSchemaFromFile(
    join(import.meta.dirname, '../graphql/server-schema.json')
)
const config: Config<ActorContext> = {
    context: {
        schema: {
            server,
            client: buildClientSchemaFromServerSchema(server)
        },
    },
    actors: [
        actors.ts.graphql.buildGraphqlActor({
            ...baseTsConfig,
            outPath: join(
                import.meta.dirname,
                './shared/graphql/generated/graphql.ts'
            ),
            onlyRequiredForOperations: false,
            scalarsMapping: {
                ...actors.ts.graphql.builtinScalarsMapping,
                ...actors.ts.graphql.additionalScalarsMapping,
            },
            importDeclarations: [],
        }),
        actors.ts.gqlClient.buildGQLClientActor({
            ...baseTsConfig,
            outPath: join(
                import.meta.dirname,
                './shared/graphql/generated/gql-client.ts'
            ),
            sdk: {
                defaultOperationReturnType: 'ExecuteResult.result',
                operationReturnTypeMapping: {},
                queriesKey: 'queries',
                mutationsKey: 'mutations',
                subscriptionsKey: 'subscriptions'
            },
            graphqlModulePath: './graphql.ts',
            importDeclarations: []
        })
    ],
}
await run(config)
