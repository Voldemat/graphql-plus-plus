import { OperationReturnType } from '../actor.js';
import ts from 'typescript';

export function generateFunctionBlock(
    isSubscription: boolean,
    returnType: OperationReturnType,
) {
    const callArgs = [
        ts.factory.createIdentifier('operation'),
        ts.factory.createIdentifier('variables'),
        ts.factory.createIdentifier('requestContext'),
    ];
    if (isSubscription) {
        callArgs.push(ts.factory.createIdentifier('controller'));
    }
    const awaitExpression = ts.factory.createAwaitExpression(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('executor'),
                isSubscription ? 'executeSubscription' : 'executeSync',
            ),
            undefined,
            callArgs,
        ),
    );
    if (returnType === 'ExecuteResult') {
        return ts.factory.createBlock(
            [ts.factory.createReturnStatement(awaitExpression)],
            true,
        );
    }
    return ts.factory.createBlock(
        [
            ts.factory.createVariableStatement(
                undefined,
                ts.factory.createVariableDeclarationList(
                    [
                        ts.factory.createVariableDeclaration(
                            'executorResult',
                            undefined,
                            undefined,
                            awaitExpression,
                        ),
                    ],
                    ts.NodeFlags.Const,
                ),
            ),
            ts.factory.createReturnStatement(
                ts.factory.createPropertyAccessChain(
                    ts.factory.createIdentifier('executorResult'),
                    undefined,
                    'result',
                ),
            ),
        ],
        true,
    );
}
