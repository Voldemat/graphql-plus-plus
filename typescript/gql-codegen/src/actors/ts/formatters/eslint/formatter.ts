import assert from 'assert';
import { Formatter } from '../../shared.js';
import { LoadESLintFuncType } from './types.js';

export async function buildESLintFormatter(
    loadFunc: LoadESLintFuncType
): Promise<Formatter> {
    const ESLint = await loadFunc({ useFlatConfig: true })
    const eslint = new ESLint({ fix: true })
    return async (code) => {
        const results = await eslint.lintText(code)
        const result = results[0]
        const formattedCode = result.output || result.source
        assert(formattedCode !== undefined)
        return formattedCode
    }
}
