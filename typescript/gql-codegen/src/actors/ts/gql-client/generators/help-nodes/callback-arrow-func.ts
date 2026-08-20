import ts from 'typescript';
import { Config, OperationReturnType } from '../../actor.js';
import { generateFunctionBlock } from '../operation-function-block.js';
import { createReturnTypeNode } from '../operation-return-type.js';

export function generateCallbackArrowFunction(
    config: Config,
    variablesTypeName: string,
    resultTypeName: string,
    isSubscription: boolean,
    returnType: OperationReturnType,
) {
    const funcArgs = [
        ts.factory.createParameterDeclaration(
            undefined,
            undefined,
            'variables',
            undefined,
            ts.factory.createTypeReferenceNode(variablesTypeName),
        ),
        ts.factory.createParameterDeclaration(
            undefined,
            undefined,
            'requestContext',
            undefined,
            ts.factory.createTypeReferenceNode('TRequestContext'),
        ),
    ];
    if (isSubscription) {
        funcArgs.push(
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                'controller',
                undefined,
                ts.factory.createTypeReferenceNode('AbortController'),
            ),
        );
    }
    return ts.factory.createArrowFunction(
        ts.factory.createModifiersFromModifierFlags(ts.ModifierFlags.Async),
        undefined,
        funcArgs,
        ts.factory.createTypeReferenceNode('Promise', [
            createReturnTypeNode(resultTypeName, returnType, isSubscription),
        ]),
        ts.factory.createToken(ts.SyntaxKind.EqualsGreaterThanToken),
        generateFunctionBlock(isSubscription, returnType),
    );
}
