export interface LintResult {
    output?: string
    source?: string
}

export interface ESLintClass {
    lintText(code: string): Promise<LintResult[]>
}

export interface ESLintClassConstructor {
    new (opts: { fix: boolean }): ESLintClass;
}

export type LoadESLintFuncType = (opts: { useFlatConfig: boolean }) =>
    Promise<ESLintClassConstructor>
