import ts from "typescript"
import { readFileSync } from "fs"

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
