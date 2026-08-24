import { z } from 'zod/v4';
import { RootSchema } from '@/schema/root.js';
import {
    fragmentSchema,
    FragmentSpecSchemaType,
} from '@/schema/client/fragment.js';
import ts from 'typescript';
import { extractFragmentSourceTextsInSpec } from './spec/shared.js';

export function generateFragmentDocumentNode(
    schema: RootSchema,
    name: string,
    fragment: z.infer<typeof fragmentSchema>,
) {
    return ts.factory.createVariableStatement(
        [ts.factory.createToken(ts.SyntaxKind.ExportKeyword)],
        ts.factory.createVariableDeclarationList(
            [
                ts.factory.createVariableDeclaration(
                    ts.factory.createIdentifier(name + 'FragmentDocument'),
                    undefined,
                    undefined,
                    ts.factory.createStringLiteral(
                        [
                            fragment.sourceText,
                            ...extractFragmentSourceTextsInSpec(
                                schema,
                                fragment.spec as FragmentSpecSchemaType,
                            ),
                        ].join(' '),
                    ),
                ),
            ],
            ts.NodeFlags.Const,
        ),
    );
}
