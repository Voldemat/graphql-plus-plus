export interface ErrorLabel {
    message: string | null;
    start: number;
    end: number;
}
export interface OxcError {
    severity: Severity;
    message: string;
    labels: Array<ErrorLabel>;
    helpMessage: string | null;
    codeframe: string | null;
}
export type Severity = 'Error' | 'Warning' | 'Advice';
export interface FormatResult {
    /** The formatted code. */
    code: string;
    /** Parse and format errors. */
    errors: Array<OxcError>;
}

export type FormatFunc<T> = (
    filename: string,
    code: string,
    options: T,
) => Promise<FormatResult>;
