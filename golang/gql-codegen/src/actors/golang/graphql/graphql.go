package graphql

import (
	"fmt"
	"gql-codegen/src/actors"
	"gql-codegen/src/actors/golang"
	"gql-codegen/src/schema"
	"slices"

	"github.com/dave/jennifer/jen"
	"go.uber.org/zap"
	"golang.org/x/text/cases"
	"golang.org/x/text/language"
)

type ScalarSpec interface {
	OnObjectType(s *jen.Statement)
	OnInputType(s *jen.Statement)
}

func applyObjectTypeSpec(
	scalarsMapping map[string]ScalarSpec,
	spec schema.ObjectTypeSpec,
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
			scalarSpec.OnObjectType(statement)
		}
	default:
		{
			statement.Id(spec.Name)
		}
	}
	return nil
}

func applyObjectFieldSpec(
	scalarsMapping map[string]ScalarSpec,
	fieldSpec schema.ObjectFieldSpec,
	statement *jen.Statement,
) error {
	switch spec := fieldSpec.Value.(type) {
	case *schema.ObjectLiteralFieldSpec:
		{
			return applyObjectTypeSpec(scalarsMapping, spec.Type, statement)
		}
	case *schema.ObjectArrayFieldSpec:
		{
			statement.Index()
			if spec.Nullable {
				statement.Op("*")
			}
			return applyObjectTypeSpec(scalarsMapping, spec.Type, statement)
		}
	case *schema.ObjectCallableFieldSpec:
		{
		}
	}
	return nil
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
			statement.Op("*")
			tag += ",omitempty"
		}
		err := applyObjectFieldSpec(scalarsMapping, field.Spec, statement)
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
		if object.Name == "Query" || object.Name == "Mutation" || object.Name == "Subscription" {
			continue
		}
		err := generateObjectTypeDefinition(actorConfig.ScalarsMapping, object, objectMapping[object.Name], file)
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
