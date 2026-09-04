/* oxlint-disable max-lines */
import { Change } from 'diff';
import pc from 'picocolors';

enum LineType {
    Added,
    Removed,
    Unchanged,
}

function buildLineNumbersString(
    newLineNumber: number,
    oldLineNumber: number,
    lineType: LineType,
): string {
    switch (lineType) {
        case LineType.Added: {
            const newStr = String(newLineNumber);
            const oldStr = ' '.repeat(newStr.length);
            return pc.green('+ ' + oldStr + ' ' + newStr);
        }
        case LineType.Removed: {
            const oldStr = String(oldLineNumber);
            const newStr = ' '.repeat(oldStr.length);
            return pc.red('- ' + oldStr + ' ' + newStr);
        }
        case LineType.Unchanged: {
            const oldStr = String(oldLineNumber);
            const newStr = String(newLineNumber);
            return pc.dim('  ' + oldStr + ' ' + newStr);
        }
    }
}

function getColorFunc(lineType: LineType): (s: string) => string {
    switch (lineType) {
        case LineType.Added:
            return pc.green;
        case LineType.Removed:
            return pc.red;
        case LineType.Unchanged:
            return pc.white;
    }
}

function formatPrefix(
    newLineNumber: number,
    oldLineNumber: number,
    lineType: LineType,
): string {
    const colorFunc = getColorFunc(lineType);
    return (
        buildLineNumbersString(newLineNumber, oldLineNumber, lineType) +
        ' ' +
        colorFunc('|')
    );
}

function formatLine(
    newLineNumber: number,
    oldLineNumber: number,
    lineType: LineType,
    line: string,
): string {
    const colorFunc = getColorFunc(lineType);
    return (
        formatPrefix(newLineNumber, oldLineNumber, lineType) +
        colorFunc(' ' + line)
    );
}

function buildSeparatorLine(
    newLineNumber: number,
    oldLineNumber: number,
): string {
    return pc
        .blue('-')
        .repeat(
            2 +
                String(newLineNumber).length +
                1 +
                String(oldLineNumber).length +
                2,
        );
}

const n = 10;

interface PrintState {
    oldLineNumber: number;
    newLineNumber: number;
    beforeBufferList: string[];
    nextN: number;
}

function printLine(state: PrintState, part: Change, line: string) {
    if (part.added) {
        if (state.beforeBufferList.length !== 0) {
            process.stderr.write(
                buildSeparatorLine(state.newLineNumber, state.oldLineNumber) +
                    '\n',
            );
            process.stderr.write(state.beforeBufferList.join('\n') + '\n');
            state.beforeBufferList = [];
        }
        state.nextN = n;

        process.stderr.write(
            formatLine(
                state.newLineNumber,
                state.oldLineNumber,
                LineType.Added,
                line,
            ) + '\n',
        );
        state.newLineNumber++;
    } else if (part.removed) {
        if (state.beforeBufferList.length !== 0) {
            process.stderr.write(
                buildSeparatorLine(state.newLineNumber, state.oldLineNumber) +
                    '\n',
            );
            process.stderr.write(state.beforeBufferList.join('\n') + '\n');
            state.beforeBufferList = [];
        }
        state.nextN = n;

        process.stderr.write(
            formatLine(
                state.newLineNumber,
                state.oldLineNumber,
                LineType.Removed,
                line,
            ) + '\n',
        );
        state.oldLineNumber++;
    } else {
        const formattedLine = formatLine(
            state.newLineNumber,
            state.oldLineNumber,
            LineType.Unchanged,
            line,
        );
        if (state.nextN !== 0) {
            process.stderr.write(formattedLine + '\n');
            state.nextN -= 1;
        } else {
            if (state.beforeBufferList.length < n) {
                state.beforeBufferList.push(formattedLine);
            } else {
                state.beforeBufferList.shift();
                state.beforeBufferList.push(formattedLine);
            }
        }
        state.oldLineNumber++;
        state.newLineNumber++;
    }
}

export function printChanges(changes: Change[]) {
    let state: PrintState = {
        oldLineNumber: 1,
        newLineNumber: 1,
        beforeBufferList: [],
        nextN: 0,
    };
    for (const part of changes) {
        const lines = part.value.replace(/\n$/, '').split('\n');

        for (const line of lines) {
            printLine(state, part, line);
        }
    }
}
