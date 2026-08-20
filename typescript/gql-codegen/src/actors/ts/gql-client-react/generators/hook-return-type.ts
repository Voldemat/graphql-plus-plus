import ts from 'typescript';
import { Config } from '../actor.js';

export function generateHookReturnType(
    config: Config,
    type: 'LAZY' | 'SYNC' | 'SUBSCRIPTION',
    variablesTypeName: string,
    resultTypeName: string,
): ts.TypeNode {
    switch (type) {
        case 'LAZY': {
            return ts.factory.createTypeReferenceNode(
                'UseLazyOperationReturnType',
                [
                    ts.factory.createTypeReferenceNode(variablesTypeName),
                    ts.factory.createTypeReferenceNode(resultTypeName),
                    ts.factory.createTypeReferenceNode('TRequestContext'),
                ],
            );
        }
        case 'SYNC': {
            return ts.factory.createTypeReferenceNode('OperationState', [
                ts.factory.createTypeReferenceNode(resultTypeName),
            ]);
        }
        case 'SUBSCRIPTION': {
            return ts.factory.createTypeReferenceNode('OperationState', [
                ts.factory.createTypeReferenceNode('SubOpAsyncIterable', [
                    ts.factory.createTypeReferenceNode(resultTypeName),
                ]),
            ]);
        }
    }
}
