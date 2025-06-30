import { entries } from '../../../../utils.js'
import { ImportModule } from '../actor.js'
import ts from 'typescript'
import assert from 'assert'

interface ImportProps {
    alias: string | null
    type: boolean
}

interface ImportModuleClause {
    imports: Record<string, ImportProps>
    identifiers: Set<string>
}

export class ImportRegistry {
    private imports: Record<ImportModule | string, ImportModuleClause> = {
        [ImportModule.APOLLO]: { imports: {}, identifiers: new Set() },
        [ImportModule.GRAPHQL]: { imports: {}, identifiers: new Set() }
    }

    public generateImportDeclarations(
        modulePaths: Record<ImportModule, string>
    ): ts.ImportDeclaration[] {
        return entries<typeof this.imports>(this.imports)
            .map(([module, clause]) => {
                return ts.factory.createImportDeclaration(
                    undefined,
                    ts.factory.createImportClause(
                        false,
                        undefined,
                        ts.factory.createNamedImports(
                            entries<typeof clause.imports>(clause.imports)
                                .map(([name, props]) =>
                                    ts.factory.createImportSpecifier(
                                        props.type,
                                        props.alias !== null ?
                                            ts.factory.createIdentifier(
                                                props.alias
                                            ) :
                                            undefined,
                                        ts.factory.createIdentifier(name)
                                    ))
                        )
                    ),
                    ts.factory.createStringLiteral(
                        (module in modulePaths ?
                            modulePaths[module as ImportModule] :
                            module) as string
                    )
                )
            })
    }

    public addImport(
        module: ImportModule | string,
        name: string,
        props: ImportProps = { alias: null, type: true }
    ) {
        this.addImports(module, { [name]: props })
        return props.alias || name
    }

    public addImports(
        module: ImportModule | string,
        imports: Record<string, ImportProps>
    ): void {
        if (!(module in this.imports)) {
            this.imports[module] = {
                imports: {},
                identifiers: new Set()
            }
        }
        const clause = this.imports[module]
        for (const [name, props] of Object.entries(imports)) {
            if (name in clause.imports) {
                if (clause.imports[name].alias === props.alias) continue
                this.addToClause(clause, name, props)
                continue
            }
            this.addToClause(clause, name, props)
        }
    }

    private addToClause(
        clause: ImportModuleClause,
        name: string,
        props: ImportProps
    ) {
        if (props.alias === null) {
            assert(!clause.identifiers.has(name))
            clause.imports[name] = props
            clause.identifiers.add(name)
            return
        }
        assert(!clause.identifiers.has(props.alias))
        clause.imports[name] = props
        clause.identifiers.add(props.alias)
    }
}
