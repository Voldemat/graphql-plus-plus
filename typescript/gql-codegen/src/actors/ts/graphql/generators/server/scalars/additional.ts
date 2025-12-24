/* eslint-disable max-lines */
import ts from 'typescript';
import { buildSymmetricScalarSpec, ScalarSpec } from './mapping.js';
import { invokeMethod } from '../../../../shared.js';
import { builtinScalarsMapping } from './builtin.js';

export const additionalScalarsMapping = {
    Int64: builtinScalarsMapping.Int,
    UUID: builtinScalarsMapping.String,
    Datetime: {
        inputSchema: invokeMethod(
            ts.factory.createIdentifier('z'),
            'date',
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
    Url: {
        inputSchema: invokeMethod(
            ts.factory.createCallExpression(
                ts.factory.createPropertyAccessExpression(
                    ts.factory.createIdentifier('z'),
                    'custom'
                ),
                [ts.factory.createTypeReferenceNode('URL')],
                []
            ),
            'transform',
            [ts.factory.createArrowFunction(
                undefined,
                undefined,
                [ts.factory.createParameterDeclaration(
                    undefined,
                    undefined,
                    'url'
                )],
                undefined,
                ts.factory.createToken(
                    ts.SyntaxKind.EqualsGreaterThanToken
                ),
                invokeMethod(
                    ts.factory.createIdentifier('url'),
                    'toString',
                    []
                )
            )]
        ),
        outputSchema: invokeMethod(
            ts.factory.createCallExpression(
                ts.factory.createPropertyAccessExpression(
                    ts.factory.createIdentifier('z'),
                    'string'
                ),
                [],
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
                    ts.factory.createIdentifier('Url'),
                    undefined,
                    [ts.factory.createIdentifier('v')]
                )
            )]
        ),
    },
    Void: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'any'
            ),
            undefined,
            []
        )
    ),
    Date: {
        inputSchema: invokeMethod(
            invokeMethod(
                ts.factory.createIdentifier('z'),
                'date',
                []
            ),
            'transform',
            [
                ts.factory.createArrowFunction(
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
                    ts.factory.createElementAccessExpression(
                        invokeMethod(
                            invokeMethod(
                                ts.factory.createIdentifier('v'),
                                'toISOString',
                                []
                            ),
                            'split',
                            [ts.factory.createStringLiteral('T')]
                        ),
                        ts.factory.createNumericLiteral(0)
                    )
                )
            ]
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
} as const satisfies Record<string, ScalarSpec> 
