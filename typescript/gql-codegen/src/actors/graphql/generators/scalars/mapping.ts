import ts from 'typescript'

export type BuiltinScalarName = 'String' | 'Boolean' | 'Int'
export interface ScalarSpec {
    input: ts.TypeNode
    output: ts.TypeNode
}
export type ScalarsMapping =
    Record<BuiltinScalarName, ScalarSpec> & Record<string, ScalarSpec>

export function buildSymmetricScalarSpec(specType: ts.TypeNode): ScalarSpec {
    return {
        input: specType,
        output: specType
    }
}

export const builtinScalarsMapping: ScalarsMapping = {
    ID: buildSymmetricScalarSpec(
        ts.factory.createTypeReferenceNode('string')
    ),
    String: buildSymmetricScalarSpec(
        ts.factory.createTypeReferenceNode('string')
    ),
    Int: buildSymmetricScalarSpec(
        ts.factory.createTypeReferenceNode('number')
    ),
    Float: buildSymmetricScalarSpec(
        ts.factory.createTypeReferenceNode('number')
    ),
    Boolean: buildSymmetricScalarSpec(
        ts.factory.createTypeReferenceNode('boolean')
    ),
}

export function getScalarSpecFromMapping(
    scalarsMapping: ScalarsMapping,
    name: string
) {
    if (!(name in scalarsMapping)) {
        throw new Error(`No scalar spec defined in mapping for scalar: ${name}`)
    }
    return scalarsMapping[name]
}
