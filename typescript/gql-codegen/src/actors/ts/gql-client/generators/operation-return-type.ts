import ts from 'typescript';
import { Config, OperationReturnType } from '../actor.js';

export function createReturnTypeNode(
    resultName: string,
    returnType: OperationReturnType,
    isSubscription: boolean,
) {
    const rOpType = ts.factory.createTypeReferenceNode(resultName);
    const rType = isSubscription
        ? ts.factory.createTypeReferenceNode('types.SubOpAsyncIterable', [
              rOpType,
          ])
        : rOpType;
    if (returnType === 'ExecuteResult.result') return rType;
    return ts.factory.createTypeReferenceNode('types.ExecuteResult', [rType]);
}

export function getReturnTypeFromConfig(config: Config, operationName: string) {
    return (
        config.sdk.operationReturnTypeMapping[operationName] ||
        config.sdk.defaultOperationReturnType
    );
}
