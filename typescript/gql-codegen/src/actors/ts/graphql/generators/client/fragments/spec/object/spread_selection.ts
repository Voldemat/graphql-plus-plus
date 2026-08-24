import ts from 'typescript';
import { invokeMethod } from '../../../../../../shared.js';
import { generateSchemaName } from '../../../../server/shared.js';

function constructObjectWithShape(
    fragmentName: string,
    isLazy: boolean,
): ts.Expression {
    const schemaName = generateSchemaName(fragmentName + 'Fragment');
    const identifier = ts.factory.createIdentifier(schemaName);
    if (!isLazy) return identifier;
    return invokeMethod(
        ts.factory.createPropertyAccessExpression(identifier, 'def'),
        'getter',
        [],
    );
}

export function generateSpreadSelection(
    fragmentName: string,
    isLazy: boolean,
): ts.SpreadAssignment {
    return ts.factory.createSpreadAssignment(
        ts.factory.createPropertyAccessExpression(
            constructObjectWithShape(fragmentName, isLazy),
            ts.factory.createIdentifier('shape'),
        ),
    );
}
