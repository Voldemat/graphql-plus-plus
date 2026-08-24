import { fragmentSchema } from '@/schema/client/fragment.js';
import { RootSchema } from '@/schema/root.js';
import ts from 'typescript';
import { z } from 'zod/v4';
import { ScalarsMapping } from '../../server/scalars/mapping.js';
import {
    generateSchemaName,
    generateZodInferInterfaceType,
    generateZodInferTypeAlias,
} from '../../server/shared.js';
import { generateFragmentDocumentNode } from './document.js';
import { generateZodFragmentSchema } from './schema.js';

function generateFragmentDeclarations(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
    fragmentName: string,
    fragment: z.infer<typeof fragmentSchema>,
): ts.Node[] {
    const fName = fragmentName + 'Fragment';
    const [fragmentSchemaNode, isLazy] = generateZodFragmentSchema(
        scalarsMapping,
        schema,
        lazyFragmentsSet,
        fragmentName,
        fragment,
    );
    if (isLazy) {
        lazyFragmentsSet.add(fragmentName);
    }
    return [
        generateFragmentDocumentNode(schema, fragmentName, fragment),
        fragmentSchemaNode,
        fragment.spec._type === 'ObjectFragmentSpec'
            ? generateZodInferInterfaceType(
                  'output',
                  fName,
                  generateSchemaName(fName),
              )
            : generateZodInferTypeAlias(
                  'output',
                  fName,
                  generateSchemaName(fName),
              ),
    ];
}

export function generateFragmentTypes(
    scalarsMapping: ScalarsMapping,
    schema: RootSchema,
    lazyFragmentsSet: Set<string>,
): ts.Node[] {
    return Object.entries(schema.client.fragments)
        .map(([name, fragment]) => {
            return generateFragmentDeclarations(
                scalarsMapping,
                schema,
                lazyFragmentsSet,
                name,
                fragment,
            );
        })
        .flat();
}
