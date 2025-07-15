package graphql

import (
	"fmt"
	"gql-codegen/src/actors"
	"gql-codegen/src/actors/golang"
	"gql-codegen/src/schema"
	"reflect"
	"slices"

	"github.com/dave/jennifer/jen"
	"go.uber.org/zap"
	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

type ScalarSpec interface {
	OnObjectType(s *jen.Statement, nullable bool)
	OnInputType(s *jen.Statement)
}

func applyObjectTypeSpec(
	scalarsMapping map[string]ScalarSpec,
	spec schema.ObjectTypeSpec,
    nullable bool,
	statement *jen.Statement,
) error {
	switch spec.Type {
	case schema.ObjectTypeScalar:
		{
			scalarSpec, ok := scalarsMapping[spec.Name]
			if !ok {
				return fmt.Errorf(
					"No scalar spec is provided for scalar: %s",
					spec.Name,
				)
			}
			scalarSpec.OnObjectType(statement, nullable)
            return nil
		}
    case schema.ObjectTypeUnion: {
        statement.Id(spec.Name)
        return nil
    }
	default:
		{
            if nullable {
                statement.Op("*")
            }
			statement.Id(spec.Name)
		}
	}
	return nil
}

func applyNonCallableObjectFieldSpec(
	scalarsMapping map[string]ScalarSpec,
	fieldSpec schema.ObjectNonCallableFieldSpec,
    nullable bool,
	statement *jen.Statement,
) error {
	switch spec := fieldSpec.Value.(type) {
	case *schema.ObjectLiteralFieldSpec:
		{
			return applyObjectTypeSpec(scalarsMapping, spec.Type, nullable, statement)
		}
	case *schema.ObjectArrayFieldSpec:
		{
            if nullable {
                statement.Op("*")
            }
			statement.Index()
			return applyObjectTypeSpec(scalarsMapping, spec.Type, spec.Nullable, statement)
		}
    }
	return fmt.Errorf("Unknown ObjectNonCallableFieldSpec, %s", reflect.TypeOf(fieldSpec.Value))
}
func applyObjectFieldSpec(
	scalarsMapping map[string]ScalarSpec,
	fieldSpec schema.ObjectFieldSpec,
    nullable bool,
	statement *jen.Statement,
) error {
	switch spec := fieldSpec.Value.(type) {
	case *schema.ObjectLiteralFieldSpec:
        return applyNonCallableObjectFieldSpec(
            scalarsMapping,
            schema.ObjectNonCallableFieldSpec{
                Value: fieldSpec.Value,
            },
            nullable,
            statement,
        )
	case *schema.ObjectArrayFieldSpec:
        return applyNonCallableObjectFieldSpec(
            scalarsMapping,
            schema.ObjectNonCallableFieldSpec{
                Value: fieldSpec.Value,
            },
            nullable,
            statement,
        )
	case *schema.ObjectCallableFieldSpec:
        return applyNonCallableObjectFieldSpec(
            scalarsMapping,
            spec.ReturnType,
            nullable,
            statement,
        )
	}
	return fmt.Errorf("Unknown ObjectFieldSpec")
}

func generateObjectTypeDefinition(
	scalarsMapping map[string]ScalarSpec,
	object schema.ObjectSchema,
	unionNames []string,
	file *jen.File,
) error {
	var statements []jen.Code = []jen.Code{}
	for name, field := range object.Fields {
		statement := jen.Id(cases.Title(language.English, cases.NoLower).String(name))
		tag := name
		if field.Nullable {
			tag += ",omitempty"
		}
		err := applyObjectFieldSpec(scalarsMapping, field.Spec, field.Nullable, statement)
		if err != nil {
			return err
		}
		statement.Tag(map[string]string{"json": tag})
		statements = append(statements, statement)
	}
	file.Type().Id(object.Name).Struct(statements...)
	for _, name := range unionNames {
		file.Func().Params(jen.Id(object.Name)).Id("Is" + name).Params().Block()
	}
	return nil
}

func generateEnumDefinition(
	enum schema.EnumSchema,
	file *jen.File,
) {
	file.Type().Id(enum.Name).String()
	statements := []jen.Code{}
	values := []jen.Code{}
	for _, value := range enum.Values {
		statements = append(statements, jen.Id(enum.Name+value).Id(enum.Name).Op("=").Lit(value))
		values = append(values, jen.Id(enum.Name+value))
	}
	file.Const().Defs(statements...)
	allValuesOfName := "allValuesOf" + enum.Name
	file.Var().Id(allValuesOfName).Op("=").Index().Id(enum.Name).Values(
		values...,
	)
	file.Func().Params(jen.Id("self").Op("*").Id(enum.Name)).Id("UnmarshalJSON").Params(jen.Id("data").Index().Id("byte")).Error().Block(
		jen.Var().Id("value").String(),
		jen.Id("err").Op(":=").Qual("encoding/json", "Unmarshal").Call(jen.Id("data"), jen.Op("&").Id("value")),
		jen.If(jen.Id("err").Op("!=").Nil()).Block(jen.Return(jen.Id("err"))),
		jen.If(jen.Op("!").Qual("slices", "Contains").Call(jen.Id(allValuesOfName), jen.Id(enum.Name).Call(jen.Id("value")))).Block(
			jen.Return(jen.Qual("fmt", "Errorf").Call(jen.Lit("Invalid "+enum.Name+" value: %s"), jen.Id("value"))),
		),
		jen.Op("*").Id("self").Op("=").Id(enum.Name).Call(jen.Id("value")),
		jen.Return(jen.Nil()),
	)
}

func generateUnionDefinition(
	objectMapping map[string][]string,
	union schema.UnionSchema,
	file *jen.File,
) {
	for item := range union.Items {
		val := objectMapping[item]
		if !slices.Contains(val, union.Name) {
			val = append(val, union.Name)
		}
		objectMapping[item] = val
	}
	file.Type().Id(union.Name).Interface(jen.Id("Is" + union.Name).Params())
}

func applyInputTypeSpec(
	scalarsMapping map[string]ScalarSpec,
	spec schema.InputTypeSpec,
	statement *jen.Statement,
) error {
	switch spec.Type {
	case schema.InputTypeScalar:
		{
			scalarSpec, ok := scalarsMapping[spec.Name]
			if !ok {
				return fmt.Errorf(
					"No scalar spec is provided for scalar: %s",
					spec.Name,
				)
			}
			scalarSpec.OnInputType(statement)
		}
	default:
		{
			statement.Id(spec.Name)
		}
	}
	return nil
}

func applyInputFieldSpec(
	scalarsMapping map[string]ScalarSpec,
	fieldSpec schema.InputFieldSpec,
	statement *jen.Statement,
) error {
	switch spec := fieldSpec.Value.(type) {
	case *schema.InputLiteralFieldSpec:
		{
			return applyInputTypeSpec(scalarsMapping, spec.Type, statement)
		}
	case *schema.InputArrayFieldSpec:
		{
			statement.Index()
			if spec.Nullable {
				statement.Op("*")
			}
			return applyInputTypeSpec(scalarsMapping, spec.Type, statement)
		}
	}
	return nil
}

func generateInputTypeDefinition(
	scalarsMapping map[string]ScalarSpec,
	input schema.InputSchema,
	file *jen.File,
) error {
	var statements []jen.Code = []jen.Code{}
	for name, field := range input.Fields {
		statement := jen.Id(cases.Title(language.English, cases.NoLower).String(name))
		tag := name
		if field.Nullable {
			statement.Op("*")
			tag += ",omitempty"
		}
		err := applyInputFieldSpec(scalarsMapping, field.Spec, statement)
		if err != nil {
			return err
		}
		statement.Tag(map[string]string{"json": tag})
		statements = append(statements, statement)
	}
	file.Type().Id(input.Name).Struct(statements...)
	return nil
}

type GraphqlActorConfig struct {
	golang.GolangActorConfig
	ScalarsMapping map[string]ScalarSpec
}

func generateObjectInterfaceDefinition(
    scalarsMapping map[string]ScalarSpec,
    object schema.ObjectSchema,
    file *jen.File,
) error {
    methods := []jen.Code{}
    for name, field := range object.Fields {
		statement := jen.Id(cases.Title(language.English, cases.NoLower).String(name)).Params()
		err := applyObjectFieldSpec(scalarsMapping, field.Spec, field.Nullable, statement)
		if err != nil {
			return err
		}
		methods = append(methods, statement)
    }
    file.Type().Id(object.Name).Interface(methods...)
    return nil
}

func generateFile(
	conf actors.IActorConfig,
	actorConfig GraphqlActorConfig,
) {
	s := conf.GetSchema()
	logger := conf.GetLogger()
	file := actorConfig.CreateJenFile()
	for _, enum := range s.Server.Enums {
		generateEnumDefinition(enum, file)
		file.Line()
	}
	objectMapping := make(map[string][]string)
	for _, union := range s.Server.Unions {
		generateUnionDefinition(objectMapping, union, file)
		file.Line()
	}
	for _, object := range s.Server.Objects {
        var err error
		if object.Name == "Query" || object.Name == "Mutation" || object.Name == "Subscription" {
            err = generateObjectInterfaceDefinition(
                actorConfig.ScalarsMapping,
                object,
                file,
            )
		} else {
            err = generateObjectTypeDefinition(actorConfig.ScalarsMapping, object, objectMapping[object.Name], file)
        }
		if err != nil {
			logger.Fatal("Failed to generate object type definition", zap.Error(err))
		}
		file.Line()
	}
	for _, input := range s.Server.Inputs {
		err := generateInputTypeDefinition(actorConfig.ScalarsMapping, input, file)
		if err != nil {
			logger.Fatal("Failed to generate input type definition", zap.Error(err))
		}
		file.Line()
	}
	err := actorConfig.SaveFile(file)
	if err != nil {
		logger.Fatal("Failed to save file", zap.Error(err))
	}
}

func BuildGraphqlActor(actorConfig GraphqlActorConfig) actors.Actor {
	return func(conf actors.IActorConfig) {
		generateFile(conf, actorConfig)
	}
}
