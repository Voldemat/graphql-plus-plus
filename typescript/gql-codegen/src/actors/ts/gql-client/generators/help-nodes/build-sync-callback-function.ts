/* oxlint-disable max-lines */
import ts from 'typescript';
import { Config, OperationReturnType } from '../../actor.js';
import { generateCallbackArrowFunction } from './callback-arrow-func.js';

export function createBuildSyncCallbackFunctionName(
    returnType: OperationReturnType,
): string {
    switch (returnType) {
        case 'ExecuteResult':
            return 'buildSyncExecuteResultCallback';
        case 'ExecuteResult.result':
            return 'buildSyncResultCallback';
    }
}

export function generateBuildSyncCallbackFunction(
    config: Config,
    returnType: OperationReturnType,
): ts.Node {
    return ts.factory.createFunctionDeclaration(
        undefined,
        undefined,
        ts.factory.createIdentifier(
            createBuildSyncCallbackFunctionName(returnType),
        ),
        [
            ts.factory.createTypeParameterDeclaration(
                undefined,
                ts.factory.createIdentifier('TExecutor'),
                ts.factory.createTypeReferenceNode('IExecutor', [
                    ts.factory.createTypeReferenceNode('TRequestContext'),
                ]),
                undefined,
            ),
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
                ts.factory.createTypeReferenceNode('TExecutor'),
                undefined,
            ),
            ts.factory.createParameterDeclaration(
                undefined,
                undefined,
                ts.factory.createIdentifier('operation'),
                undefined,
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('SyncOperation'),
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
            ts.factory.createIdentifier(config.sdk.gqlSyncMethodFuncTypeName),
            [
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('TRequestContext'),
                    undefined,
                ),
                ts.factory.createTypeReferenceNode(
                    ts.factory.createIdentifier('V'),
                    undefined,
                ),
                returnType === 'ExecuteResult.result'
                    ? ts.factory.createTypeReferenceNode(
                          ts.factory.createIdentifier('R'),
                      )
                    : ts.factory.createTypeReferenceNode('ExecuteResult', [
                          ts.factory.createTypeReferenceNode('R'),
                      ]),
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
