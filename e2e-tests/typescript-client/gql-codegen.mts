/* eslint-disable max-lines */
import { join } from "path";
import {
    type ActorContext,
    actors,
    type Config,
    run,
    runActionFromArgv,
} from "@vladimirdev635/gql-codegen";
import { loadRootSchemaFromGQLSubprocess } from "@vladimirdev635/gql-codegen/schema/utils";
import { format } from "oxfmt";
import oxfmtConfig from "./oxfmt.config.mts";
import type { OperationType } from "@vladimirdev635/gql-codegen/schema/client/operation";

const oxfmtFormatter = await actors.ts.formatters.oxfmt.build(
    format,
    oxfmtConfig,
);
const baseTsConfig: actors.ts.TSActorConfig = {
    tsconfigCompilerOptions: actors.ts.loadTsConfigCompilerOptions(),
    formatters: [
        async (code) =>
            "/* oxlint-disable no-use-before-define,max-lines */\n" + code,
        oxfmtFormatter,
    ],
};
const clientTypeNameBuilders: actors.ts.ClientTypeNameBuilders = {
    operationTypeName: (name) => name + "Operation",
    variablesTypeName: (name) => name + "Variables",
    resultTypeName: (name) => name + "Result",
};
const config: Config<ActorContext> = {
    context: {
        schema: await loadRootSchemaFromGQLSubprocess(),
    },
    actors: [
        actors.ts.graphql.build({
            ...baseTsConfig,
            clientTypeNameBuilders,
            outPath: join(
                import.meta.dirname,
                "./shared/graphql/generated/graphql.ts",
            ),
            onlyRequiredForOperations: false,
            scalarsMapping: {
                ...actors.ts.graphql.builtinScalarsMapping,
                ...actors.ts.graphql.additionalScalarsMapping,
            },
            importDeclarations: [],
        }),
        actors.ts.gqlClient.build({
            ...baseTsConfig,
            outPath: join(
                import.meta.dirname,
                "./shared/graphql/generated/gql-client.ts",
            ),
            sdk: {
                defaultOperationReturnType: "ExecuteResult.result",
                operationReturnTypeMapping: {},
                queriesKey: "queries",
                mutationsKey: "mutations",
                subscriptionsKey: "subscriptions",
                clientTypeNameBuilders,
                gqlSyncMethodFuncTypeName: "GQLSyncMethodFuncType",
                gqlSubscriptionMethodFuncTypeName:
                    "GQLSubscriptionMethodFuncType",
                typeName: "SdkType",
                operationRequestsTypeNameBuilder: (
                    operationType: OperationType,
                ) => {
                    switch (operationType) {
                        case "QUERY":
                            return "GQLQueryRequests";
                        case "MUTATION":
                            return "GQLMutationRequests";
                        case "SUBSCRIPTION":
                            return "GQLSubscriptionRequests";
                    }
                },
            },
            graphqlModulePath: "./graphql.ts",
            importDeclarations: [],
        }),
    ],
};
await run(config, runActionFromArgv(process.argv));
