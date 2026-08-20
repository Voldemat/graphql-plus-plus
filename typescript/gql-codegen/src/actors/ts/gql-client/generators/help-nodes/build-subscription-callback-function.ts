/* oxlint-disable max-lines */
import ts from 'typescript';
import { Config, OperationReturnType } from '../../actor.js';
import { generateCallbackArrowFunction } from './callback-arrow-func.js';

export function createBuildSubscriptionCallbackFunctionName(
    returnType: OperationReturnType,
): string {
    switch (returnType) {
        case 'ExecuteResult':
            return 'buildSubscriptionExecuteResultCallback';
        case 'ExecuteResult.result':
            return 'buildSubscriptionResultCallback';
    }
}

export function generateBuildSubscriptionCallbackFunction(
    config: Config,
    returnType: OperationReturnType,
): ts.Node {
    return ts.factory.createFunctionDeclaration(
        undefined,
        undefined,
        ts.factory.createIdentifier(
            createBuildSubscriptionCallbackFunctionName(returnType),
        ),
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('TRequestContext'),
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('RequestContext'),
                    undefined,
                ),
                undefined,
            ),
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('V'),
                undefined,
                undefined,
            ),
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('R'),
                undefined,
                undefined,
            ),
        ],
        [
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                ts.factory.createIdentifier('executor'),
                undefined,
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('IExecutor'),
                    [
                        ts.factory.createTypeReferenceNode(
                            ts.factory.createIdentifier('TRequestContext'),
                            undefined,
                        ),
                    ],
                ),
                undefined,
            ),
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                ts.factory.createIdentifier('operation'),
                undefined,
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('SubscriptionOperation'),
                    [
                        ts.factory.createTypeReferenceNode(
                            ts.factory.createIdentifier('V'),
                            undefined,
                        ),
                        ts.factory.createTypeReferenceNode(
                            ts.factory.createIdentifier('R'),
                            undefined,
                        ),
                    ],
                ),
                undefined,
            ),
        ],
        ts.factory.createTypeReferenceNode(
            ts.factory.createIdentifier(config.sdk.gqlMethodFuncTypeName),
            [
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('TRequestContext'),
                    undefined,
                ),
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('V'),
                    undefined,
                ),
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('SubOpAsyncIterable'),
                    [
                        returnType === 'ExecuteResult.result'
                            ? ts.factory.createTypeReferenceNode(
                                  ts.factory.createIdentifier('R'),
                              )
                            : ts.factory.createTypeReferenceNode(
                                  'ExecuteResult',
                                  [ts.factory.createTypeReferenceNode('R')],
                              ),
                    ],
                ),
            ],
        ),
        ts.factory.createBlock(
            [
                ts.factory.createReturnStatement(
                    generateCallbackArrowFunction(
                        config,
                        'V',
                        'R',
                        false,
                        returnType,
                    ),
                ),
            ],
            true,
        ),
    );
}
