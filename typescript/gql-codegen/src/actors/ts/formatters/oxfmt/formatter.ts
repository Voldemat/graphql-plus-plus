/* oxlint-disable no-console */
import { Formatter } from '../../shared.js';
import { FormatFunc } from './types.js';

export async function build<T>(
    formatFunc: FormatFunc<T>,
    options: T,
): Promise<Formatter> {
    return async (code) => {
        const result = await formatFunc('[buffer].ts', code, options);
        for (const error of result.errors) {
            if (error.codeframe !== null) {
                console.error(error.codeframe);
            } else {
                console.error(error.message);
                if (error.helpMessage !== null) {
                    console.error(error.message);
                }
            }
        }
        if (result.errors.length > 0) {
            throw new Error('Oxfmt failed to format code');
        }
        return result.code;
    };
}
