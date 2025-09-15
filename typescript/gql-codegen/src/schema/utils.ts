/* eslint-disable max-lines */
import { PathOrFileDescriptor, readFileSync } from 'fs';
import {
    objectFieldSchema,
    objectFieldSpecSchema,
    objectNonCallableFieldSpecSchema,
    objectSchema,
    ServerSchema,
    serverSchema,
    unionSchema
} from './server.js';
import { ClientSchema, clientSchema } from './client/root.js';
import { z } from 'zod/v4';
import { RootSchema } from './root.js';
import { exec } from 'node:child_process'
import {
    fieldSelection,
    fragmentSchema,
    fragmentSpecSchema,
    objectConditionalSpreadSelection
} from './client/fragment.js';
import { inputFieldSchema } from './shared.js';
import { argument } from './client/argument.js';
import { operationSchema } from './client/operation.js';

export function loadSchemaFromFile<T extends z.ZodType>(
    p: PathOrFileDescriptor,
    schema: T
): z.infer<T> {
    return schema.parse(JSON.parse(readFileSync(p).toString()))
}

export function loadServerSchemaFromFile(
    p: PathOrFileDescriptor
): ServerSchema {
    return loadSchemaFromFile(p, serverSchema);
}

export function loadClientSchemaFromFile(
    p: PathOrFileDescriptor
): ClientSchema {
    return loadSchemaFromFile(p, clientSchema);
}

export async function loadServerSchemaFromGQLSubprocess(
    gqlPath: string = 'gql',
    gqlArgs: string[] = ['generate']
): Promise<ServerSchema> {
    const output = await new Promise<string>((resolve, reject) => {
        exec([gqlPath, ...gqlArgs].join(' '), (err, stdout, stderr) => {
            if (err !== null) {
                reject(err);
            };
            if (stderr !== '') {
                reject(err);
            };
            resolve(stdout.split('\n').filter(s => s !== '')[0]);
        })
    });
    return serverSchema.parse(JSON.parse(output))
}

export async function loadRootSchemaFromGQLSubprocess(
    gqlPath: string = 'gql',
    gqlArgs: string[] = ['generate']
): Promise<RootSchema> {
    const output = await new Promise<string[]>((resolve, reject) => {
        exec([gqlPath, ...gqlArgs].join(' '), (err, stdout, stderr) => {
            if (err !== null) {
                reject(err);
            };
            if (stderr !== '') {
                reject(err);
            };
            resolve(stdout.split('\n').filter(s => s !== ''));
        })
    });
    return {
        server: serverSchema.parse(JSON.parse(output[0])),
        client: clientSchema.parse(JSON.parse(output[1])),
    };
}

function buildFragmentSpecFromUnion(
    union: z.infer<typeof unionSchema>
): z.infer<typeof fragmentSpecSchema> {
    return {
        _type: 'UnionFragmentSpec',
        name: union.name,
        selections: [
            { _type: 'TypenameField', alias: null },
            ...Object.keys(union.items).map(
                (item): z.infer<typeof objectConditionalSpreadSelection> =>
                    ({
                        _type: 'ObjectConditionalSpreadSelection',
                        object: item,
                        spec: {
                            _type: 'ObjectFragmentSpec',
                            name: item,
                            selections: [
                                { _type: 'SpreadSelection', fragment: item }
                            ]
                        }
                    })
            )
        ]
    }
}

function generateFragmentText(name: string, specText: string) {
    return `fragment ${name} on ${name}` + specText
}

export function buildFragmentFromUnion(
    union: z.infer<typeof unionSchema>
): z.infer<typeof fragmentSchema> {
    return {
        spec: buildFragmentSpecFromUnion(union),
        sourceText: generateFragmentText(
            union.name,
            // eslint-disable-next-line no-use-before-define
            generateUnionFragmentSpecText(union)
        )
    };
};

function extractFieldFromNonCallableFieldSpec(
    spec: z.infer<typeof objectNonCallableFieldSpecSchema>
): string {
    switch (spec.type._type) {
    case 'Enum': return ''
    case 'Scalar': return ''
    case 'Union':
    case 'ObjectType':
    case 'InterfaceType':
        return '{...' + spec.type.name + '}'
    };
};

function buildInputField(field: z.infer<typeof inputFieldSchema>): string {
    let str = ''
    if (field.spec._type === 'array') {
        str += '['
    };
    str += field.spec.type.name
    if (field.spec._type === 'array') {
        if (!field.spec.nullable) str += '!';
        str += ']'
    };
    if (!field.nullable) {
        str += '!';
    };
    return str;
};

function extractTypeFromObjectFieldSpec(
    spec: z.infer<typeof objectFieldSpecSchema>
) {
    switch (spec._type) {
    case 'literal':
    case 'array':
        return spec.type
    case 'callable':
        return spec.returnType.type
    }
}

function generateObjectFragmentSpecText(
    object: z.infer<typeof objectSchema>,
    onlyField: string | null = null
): string {
    return '{' + (onlyField !== null ? '' : '__typename ') +
        Object.entries(object.fields)
            .filter(([name]) => onlyField === null || onlyField === name)
            // eslint-disable-next-line array-callback-return
            .map(([name, field]) => {
                switch (field.spec._type) {
                case 'array':
                case 'literal': {
                    const fieldText = extractFieldFromNonCallableFieldSpec(
                        field.spec
                    )
                    return name + '' + fieldText

                }
                case 'callable': {
                    const fieldText =
                            extractFieldFromNonCallableFieldSpec(
                                field.spec.returnType
                            )
                    return [
                        `${name}(` +
                            Object.keys(
                                field.spec.arguments
                            ).map(aName => `${aName}:$${aName}`).join(',') +
                            ')',
                        fieldText
                    ].join('')
                }
                };
            }).join(' ') +
        '}'
}

function generateUnionFragmentSpecText(
    union: z.infer<typeof unionSchema>
): string {
    return (
        '{__typename' +
        Object.keys(union.items).map(item =>
            `... on ${item}{...${item}}`).join(' ') +
        '}'
    )
}

export function buildSelectionFromFieldSpec(
    spec: z.infer<typeof objectFieldSpecSchema>
): z.infer<typeof fragmentSpecSchema> | null {
    const type = extractTypeFromObjectFieldSpec(spec)
    if (type._type === 'Enum' || type._type === 'Scalar') return null
    switch (type._type) {
    case 'ObjectType':
    case 'InterfaceType':
        return {
            _type: 'ObjectFragmentSpec',
            name: type.name,
            selections: [
                { _type: 'SpreadSelection', fragment: type.name }
            ]
        }
    case 'Union':
        return {
            _type: 'UnionFragmentSpec',
            name: type.name,
            selections: [
                { _type: 'SpreadSelection', fragment: type.name }
            ]
        }
    }
}

export function buildArgumentFromFieldSpec(
    spec: z.infer<typeof objectFieldSpecSchema>
): Record<string, z.infer<typeof argument>> {
    switch (spec._type) {
    case 'array':
    case 'literal': return {}
    case 'callable':
        return Object.fromEntries(
            Object.keys(spec.arguments).map(name => {
                return [name, {
                    name,
                    value: {
                        _type: 'ref',
                        name: '$' + name
                    }
                }]
            })
        )
    }
}

export function buildFieldSelection(
    name: string,
    field: z.infer<typeof objectFieldSchema>
): z.infer<typeof fieldSelection> {
    return {
        _type: 'FieldSelection',
        name,
        alias: name,
        arguments: buildArgumentFromFieldSpec(field.spec),
        selection: buildSelectionFromFieldSpec(field.spec)
    }
};

function buildFragmentSpecFromObject(
    object: z.infer<typeof objectSchema>
): z.infer<typeof fragmentSpecSchema> {
    return {
        _type: 'ObjectFragmentSpec',
        name: object.name,
        selections: [
            { _type: 'TypenameField', alias: null },
            ...Object.entries(object.fields).map(
                ([name, field]) => buildFieldSelection(name, field)
            )
        ]
    }
}

export function buildFragmentFromObject(
    object: z.infer<typeof objectSchema>
): z.infer<typeof fragmentSchema> {
    return {
        spec: buildFragmentSpecFromObject(object),
        sourceText: generateFragmentText(
            object.name,
            generateObjectFragmentSpecText(object)
        )
    };
};

function buildParameters(field: z.infer<typeof objectFieldSpecSchema>) {
    const parameters: Record<string, z.infer<typeof inputFieldSchema>> = {}
    if (field._type === 'callable') {
        for (const [name, arg] of Object.entries(field.arguments)) {
            parameters['$' + name] = arg
        }
    }
    return parameters
}

function buildOperationSourceText(
    opType: 'QUERY' | 'MUTATION' | 'SUBSCRIPTION',
    object: z.infer<typeof objectSchema>,
    name: string,
    fieldName: string,
    spec: z.infer<typeof objectFieldSpecSchema>
): string {
    let argumentsText = ''
    if (spec._type === 'callable') {
        argumentsText += '('
        argumentsText += Object.entries(spec.arguments).map(([aName, arg]) => {
            return `$${aName}:${buildInputField(arg)}`
        }).join(',')
        argumentsText += ')'
    }
    return opType.toLowerCase() + ' ' + name + argumentsText +
        generateObjectFragmentSpecText(object, fieldName)
}

function buildFragmentSpecFromField(
    opType: 'QUERY' | 'MUTATION' | 'SUBSCRIPTION',
    name: string,
    field: z.infer<typeof objectFieldSchema>
): z.infer<typeof fragmentSpecSchema> {
    return {
        _type: 'ObjectFragmentSpec',
        name: opType[0] + opType.slice(1).toLowerCase(),
        selections: [
            {
                _type: 'FieldSelection',
                name,
                alias: name,
                arguments: buildArgumentFromFieldSpec(field.spec),
                selection: buildSelectionFromFieldSpec(field.spec)
            }
        ]
    }
}

function buildOperation(
    type: 'QUERY' | 'MUTATION' | 'SUBSCRIPTION',
    object: z.infer<typeof objectSchema>,
    fieldName: string,
    field: z.infer<typeof objectFieldSchema>
): z.infer<typeof operationSchema> {
    const name = fieldName[0].toUpperCase() + fieldName.slice(1)
    return {
        type,
        name,
        parameters: buildParameters(field.spec),
        sourceText: buildOperationSourceText(
            type, object, name, fieldName, field.spec
        ),
        fragmentSpec: buildFragmentSpecFromField(type, fieldName, field),
    }
}

function buildOperations(
    server: ServerSchema
): Record<string, z.infer<typeof operationSchema>> {
    const operations: Record<string, z.infer<typeof operationSchema>> = {}
    if ('Query' in server.objects) {
        const query = server.objects['Query']
        for (const [name, field] of Object.entries(query.fields)) {
            operations[name] = buildOperation('QUERY', query, name, field)
        }
    }
    if ('Mutation' in server.objects) {
        const mutation = server.objects['Mutation']
        for (const [name, field] of Object.entries(mutation.fields)) {
            operations[name] = buildOperation('MUTATION', mutation, name, field)
        }
    }
    if ('Subscription' in server.objects) {
        const subscription = server.objects['Subscription']
        for (const [name, field] of Object.entries(subscription.fields)) {
            operations[name] = buildOperation(
                'SUBSCRIPTION', subscription, name, field
            )
        }
    }
    return operations
}

export function buildClientSchemaFromServerSchema(
    server: ServerSchema
): ClientSchema {
    return {
        fragments: Object.fromEntries([
            ...Object.values(server.unions)
                .map(union => [
                    union.name,
                    buildFragmentFromUnion(union)
                ]),
            ...Object.values(server.objects)
                .filter(object =>
                    object.name !== 'Query' &&
                    object.name !== 'Mutation' &&
                    object.name !== 'Subscription').map(object =>
                    [object.name, buildFragmentFromObject(object)]),
        ]),
        operations: buildOperations(server),
        directives: {}
    };
};
