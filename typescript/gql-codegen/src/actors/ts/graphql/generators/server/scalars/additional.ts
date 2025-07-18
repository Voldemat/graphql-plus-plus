import ts from 'typescript';
import { buildSymmetricScalarSpec, ScalarSpec } from './mapping.js';
import { invokeMethod } from '../../../../shared.js';
import { builtinScalarsMapping } from './builtin.js';

export const additionalScalarsMapping = {
    Int64: builtinScalarsMapping.Int,
    UUID: builtinScalarsMapping.String,
    Datetime: {
        inputSchema: ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'date',
            ),
            undefined,
            []
        ),
        outputSchema: invokeMethod(
            invokeMethod(
                ts.factory.createIdentifier('z'),
                'string',
                []
            ),
            'transform',
            [ts.factory.createArrowFunction(
                undefined,
                undefined,
                [ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'v'
                )],
                undefined,
                ts.factory.createToken(
                    ts.SyntaxKind.EqualsGreaterThanToken
                ),
                ts.factory.createNewExpression(
                    ts.factory.createIdentifier('Date'),
                    [],
                    [ts.factory.createIdentifier('v')]
                )
            )]
        )
    },
    Upload: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'file'
            ),
            undefined,
            []
        )
    ),
    Url: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'custom'
            ),
            [ts.factory.createTypeReferenceNode('URL')],
            []
        )
    ),
    Void: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'never'
            ),
            undefined,
            []
        )
    ),
} as const satisfies Record<string, ScalarSpec> 
