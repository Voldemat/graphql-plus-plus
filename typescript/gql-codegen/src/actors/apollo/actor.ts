import ts from "typescript"
import { Actor, ActorContext } from "../../config.js"
import { mkdirSync, writeFileSync } from "fs"
import { join } from "path"

export interface ApolloActorConfig {
    tsconfigCompilerOptions: ts.CompilerOptions
    outDir: string
}

function apolloActor(config: ApolloActorConfig, context: ActorContext) {
    const printer = ts.createPrinter(config.tsconfigCompilerOptions)
    const sourceFile = ts.createSourceFile(
        '',
        '',
        config.tsconfigCompilerOptions.target || ts.ScriptTarget.Latest,
        false,
        ts.ScriptKind.TS
    )
    const nodes = Object.entries(
        context.schema.server.objects
    ).map(([key, value]) => {
        return ts.factory.createVariableStatement(
            [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
            ts.factory.createVariableDeclarationList([
                ts.factory.createVariableDeclaration(
                    key,
                    undefined,
                    undefined,
                    ts.factory.createStringLiteral(value.name)
                )
            ], ts.NodeFlags.Const)
        )
    })
    const code = printer.printList(
        ts.ListFormat.MultiLine,
        ts.factory.createNodeArray(nodes),
        sourceFile
    )
    mkdirSync(config.outDir, { recursive: true });
    writeFileSync(join(config.outDir, "check.ts"), code)
}

export function buildApolloActor(
    config: ApolloActorConfig
): Actor<ActorContext> {
    return context => apolloActor(config, context)
}
