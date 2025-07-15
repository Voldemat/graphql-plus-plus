export { buildGraphqlActor, GraphqlActorConfig } from './actor.js'
export {
    builtinScalarsMapping,
    additionalScalarsMapping,
    type BuiltinScalarName,
    type ScalarSpec,
    type ScalarsMapping,
    buildSymmetricScalarSpec,
    getScalarSpecFromMapping
} from './generators/server/scalars/index.js'
