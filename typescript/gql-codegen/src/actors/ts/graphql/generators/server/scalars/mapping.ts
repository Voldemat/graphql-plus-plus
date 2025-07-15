import ts from 'typescript'

export type BuiltinScalarName = 'String' | 'Boolean' | 'Int' | 'Float'
export interface ScalarSpec {
    inputSchema: ts.Expression
    outputSchema: ts.Expression
}
export type ScalarsMapping =
    Record<BuiltinScalarName, ScalarSpec> & Record<string, ScalarSpec>

export function buildSymmetricScalarSpec(
    specType: ts.Expression
): ScalarSpec {
    return {
        inputSchema: specType,
        outputSchema: specType
    }
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
