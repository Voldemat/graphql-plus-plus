import ts from 'typescript';
import {
    builtinScalarsMapping,
    getScalarSpecFromMapping,
    ScalarsMapping
} from './mapping.js';

export function generateScalarReference(name: string) {
    return ts.factory.createIndexedAccessTypeNode(
        ts.factory.createIndexedAccessTypeNode(
            ts.factory.createTypeReferenceNode('Scalars'),
            ts.factory.createLiteralTypeNode(
                ts.factory.createStringLiteral(name)
            )
        ),
        ts.factory.createLiteralTypeNode(
            ts.factory.createStringLiteral('output')
        )
    )
}

export function generateScalarsInterfaceDefinition(
    scalarsMapping: ScalarsMapping,
    scalars: string[],
) {
    return ts.factory.createInterfaceDeclaration(
        ts.factory.createModifiersFromModifierFlags(
            ts.ModifierFlags.Export
        ),
        'Scalars',
        undefined,
        undefined,
        [...Object.keys(builtinScalarsMapping), ...scalars].map(name => {
            const scalarSpec = getScalarSpecFromMapping(scalarsMapping, name)
            return ts.factory.createPropertySignature(
                undefined,
                name,
                undefined,
                ts.factory.createTypeLiteralNode([
                    ts.factory.createPropertySignature(
                        undefined,
                        'input',
                        undefined,
                        scalarSpec.input
                    ),
                    ts.factory.createPropertySignature(
                        undefined,
                        'output',
                        undefined,
                        scalarSpec.output
                    )
                ])
            )
        })
    )
}
