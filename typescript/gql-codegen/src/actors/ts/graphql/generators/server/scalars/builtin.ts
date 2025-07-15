import ts from 'typescript';
import { buildSymmetricScalarSpec, ScalarsMapping } from './mapping.js';

export const builtinScalarsMapping: ScalarsMapping = {
    ID: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'string'
            ),
            undefined,
            []
        )
    ),
    String: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'string'
            ),
            undefined,
            []
        )
    ),
    Int: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'number'
            ),
            undefined,
            []
        )
    ),
    Float: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'number'
            ),
            undefined,
            []
        )
    ),
    Boolean: buildSymmetricScalarSpec(
        ts.factory.createCallExpression(
            ts.factory.createPropertyAccessExpression(
                ts.factory.createIdentifier('z'),
                'boolean'
            ),
            undefined,
            []
        )
    ),
}
