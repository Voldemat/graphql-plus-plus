import { readFileSync } from 'fs'
import ts from 'typescript'

export type Formatter = (
    code: string,
) => Promise<string> | string
export interface TSActorConfig {
    tsconfigCompilerOptions: ts.CompilerOptions
    formatters: Formatter[]
}

export async function renderNodes(config: TSActorConfig, nodes: ts.Node[]) {
    const printer = ts.createPrinter(config.tsconfigCompilerOptions)
    const sourceFile = ts.createSourceFile(
        '',
        '',
        config.tsconfigCompilerOptions.target || ts.ScriptTarget.Latest,
        false,
        ts.ScriptKind.TS
    )
    const code = printer.printList(
        ts.ListFormat.MultiLine,
        ts.factory.createNodeArray(nodes),
        sourceFile
    )
    return await config.formatters.reduce(
        async (prevCode, formatter) => {
            const currentCode = await prevCode
            return await formatter(currentCode)
        },
        Promise.resolve(code)
    )
}

export function loadTsConfigCompilerOptions(
    configPath: string = './tsconfig.json'
) {
    const tsconfig = ts.readConfigFile(
        configPath,
        (p: string) => readFileSync(p).toString()
    )
    const config = ts.parseJsonConfigFileContent(
        tsconfig.config!,
        ts.sys,
        configPath
    )
    return config.options
}


export function addNewLineBetweenNodes(nodes: ts.Node[]): ts.Node[] {
    const finalNodes: ts.Node[] = []
    for (const node of nodes) {
        finalNodes.push(node)
        finalNodes.push(ts.factory.createIdentifier('\n'))
    }
    return finalNodes
} 
