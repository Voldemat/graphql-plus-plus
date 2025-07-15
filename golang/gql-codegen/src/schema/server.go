package schema

import (
	"encoding/json"
	"fmt"
	"slices"
)

type InputType string

const (
	InputTypeInput  InputType = "InputType"
	InputTypeEnum   InputType = "Enum"
	InputTypeScalar InputType = "Scalar"
)

var allValuesOfInputType = []InputType{
	InputTypeInput,
	InputTypeEnum,
	InputTypeScalar,
}

func (t *InputType) UnmarshalJSON(data []byte) error {
	var value string
	err := json.Unmarshal(data, &value)
	if err != nil {
		return err
	}
	if !slices.Contains(allValuesOfInputType, InputType(value)) {
		return fmt.Errorf("Invalid InputType: %s", value)
	}
	*t = InputType(value)
	return nil
}

type InputTypeSpec struct {
	Type InputType `json:"_type"`
	Name string    `json:"name"`
}

type InputLiteralFieldSpec struct {
	Type InputTypeSpec `json:"type"`
}

type InputArrayFieldSpec struct {
	Type     InputTypeSpec `json:"type"`
	Nullable bool          `json:"nullable"`
}

type InputFieldType string

const (
	InputFieldTypeArray   InputFieldType = "array"
	InputFieldTypeLiteral InputFieldType = "literal"
)

var allValuesOfInputFieldType = []InputFieldType{
	InputFieldTypeArray,
	InputFieldTypeLiteral,
}

func (t *InputFieldType) UnmarshalJSON(data []byte) error {
	var value string
	err := json.Unmarshal(data, &value)
	if err != nil {
		return err
	}
	if !slices.Contains(allValuesOfInputFieldType, InputFieldType(value)) {
		return fmt.Errorf("Invalid InputFieldType: %s", value)
	}
	*t = InputFieldType(value)
	return nil
}

type InputFieldSpec struct {
	Value any
}

type inputFieldSpecTypeStruct struct {
	Type InputFieldType `json:"_type"`
}

func (s *InputFieldSpec) UnmarshalJSON(bytes []byte) error {
	var typeStruct inputFieldSpecTypeStruct
	err := json.Unmarshal(bytes, &typeStruct)
	if err != nil {
		return err
	}
	var spec any
	switch typeStruct.Type {
	case InputFieldTypeLiteral:
		spec = new(InputLiteralFieldSpec)
	case InputFieldTypeArray:
		spec = new(InputArrayFieldSpec)
	default:
		return fmt.Errorf("Unknown fieldSpec type: %s", typeStruct.Type)
	}
	err = json.Unmarshal(bytes, &spec)
	if err != nil {
		return err
	}
	s.Value = spec
	return nil
}

type InputFieldSchema struct {
	Nullable bool           `json:"nullable"`
	Spec     InputFieldSpec `json:"spec"`
}

type ObjectType string

const (
	ObjectTypeObject    ObjectType = "ObjectType"
	ObjectTypeInterface ObjectType = "InterfaceType"
	ObjectTypeScalar    ObjectType = "Scalar"
	ObjectTypeUnion     ObjectType = "Union"
	ObjectTypeEnum      ObjectType = "Enum"
)

var allValuesOfObjectType = []ObjectType{
	ObjectTypeObject,
	ObjectTypeInterface,
	ObjectTypeScalar,
	ObjectTypeUnion,
	ObjectTypeEnum,
}

func (t *ObjectType) UnmarshalJSON(data []byte) error {
	var value string
	err := json.Unmarshal(data, &value)
	if err != nil {
		return err
	}
	if !slices.Contains(allValuesOfObjectType, ObjectType(value)) {
		return fmt.Errorf("Invalid ObjectType: %s", value)
	}
	*t = ObjectType(value)
	return nil
}

type ObjectTypeSpec struct {
	Type ObjectType `json:"_type"`
	Name string     `json:"name"`
}

type ObjectFieldType string

const (
	ObjectFieldTypeArray    ObjectFieldType = "array"
	ObjectFieldTypeLiteral  ObjectFieldType = "literal"
	ObjectFieldTypeCallable ObjectFieldType = "callable"
)

var allValuesOfObjectFieldType = []ObjectFieldType{
	ObjectFieldTypeArray,
	ObjectFieldTypeLiteral,
	ObjectFieldTypeCallable,
}

func (t *ObjectFieldType) UnmarshalJSON(data []byte) error {
	var value string
	err := json.Unmarshal(data, &value)
	if err != nil {
		return err
	}
	if !slices.Contains(allValuesOfObjectFieldType, ObjectFieldType(value)) {
		return fmt.Errorf("Invalid ObjectFieldType: %s", value)
	}
	*t = ObjectFieldType(value)
	return nil
}

type ObjectFieldSpec struct {
	Value any
}

type objectFieldSpecTypeStruct struct {
	Type ObjectFieldType `json:"_type"`
}

func (s *ObjectFieldSpec) UnmarshalJSON(bytes []byte) error {
	var typeStruct objectFieldSpecTypeStruct
	err := json.Unmarshal(bytes, &typeStruct)
	if err != nil {
		return err
	}
	var spec any
	switch typeStruct.Type {
	case ObjectFieldTypeLiteral:
		spec = new(ObjectLiteralFieldSpec)
	case ObjectFieldTypeArray:
		spec = new(ObjectArrayFieldSpec)
	case ObjectFieldTypeCallable:
		spec = new(ObjectCallableFieldSpec)
	default:
		return fmt.Errorf("Unknown fieldSpec type: %s", typeStruct.Type)
	}
	err = json.Unmarshal(bytes, &spec)
	if err != nil {
		return err
	}
	s.Value = spec
	return nil
}

type ObjectLiteralFieldSpec struct {
	Type ObjectTypeSpec `json:"type"`
}

type ObjectArrayFieldSpec struct {
	Type     ObjectTypeSpec `json:"type"`
	Nullable bool           `json:"nullable"`
}

type ObjectNonCallableFieldType string

const (
	ObjectNonCallableFieldTypeArray   ObjectNonCallableFieldType = "array"
	ObjectNonCallableFieldTypeLiteral ObjectNonCallableFieldType = "literal"
)

type ObjectNonCallableFieldSpec struct {
	json.RawMessage
	Type ObjectNonCallableFieldType `json:"_type"`
}

type ObjectCallableFieldSpec struct {
	ReturnType ObjectNonCallableFieldSpec  `json:"returnType"`
	Arguments  map[string]InputFieldSchema `json:"arguments"`
}

type ObjectFieldSchema struct {
	Nullable bool            `json:"nullable"`
	Spec     ObjectFieldSpec `json:"spec"`
}

type ObjectSchema struct {
	Name       string                       `json:"name"`
	Implements map[string]string            `json:"implements"`
	Fields     map[string]ObjectFieldSchema `json:"fields"`
}

type UnionSchema struct {
	Name  string            `json:"name"`
	Items map[string]string `json:"items"`
}

type InputSchema struct {
	Name   string                      `json:"name"`
	Fields map[string]InputFieldSchema `json:"fields"`
}

type DirectiveLocation string

const (
	DirectiveLocationSchema               DirectiveLocation = "SCHEMA"
	DirectiveLocationScalar               DirectiveLocation = "SCALAR"
	DirectiveLocationObject               DirectiveLocation = "OBJECT"
	DirectiveLocationFieldDefinition      DirectiveLocation = "FIELD_DEFINITION"
	DirectiveLocationArgumentDefinition   DirectiveLocation = "ARGUMENT_DEFINITION"
	DirectiveLocationInterface            DirectiveLocation = "INTERFACE"
	DirectiveLocationUnion                DirectiveLocation = "UNION"
	DirectiveLocationEnum                 DirectiveLocation = "ENUM"
	DirectiveLocationEnumValue            DirectiveLocation = "ENUM_VALUE"
	DirectiveLocationInputObject          DirectiveLocation = "INPUT_OBJECT"
	DirectiveLocationInputFieldDefinition DirectiveLocation = "INPUT_FIELD_DEFINITION"
)

type DirectiveSchema struct {
	Name      string              `json:"name"`
	Locations []DirectiveLocation `json:"locations"`
}

type EnumSchema struct {
	Name   string   `json:"name"`
	Values []string `json:"values"`
}

type ServerSchema struct {
	Objects    map[string]ObjectSchema    `json:"objects"`
	Directives map[string]DirectiveSchema `json:"directives"`
	Unions     map[string]UnionSchema     `json:"unions"`
	Enums      map[string]EnumSchema      `json:"enums"`
	Scalars    []string                   `json:"scalars"`
	Inputs     map[string]InputSchema     `json:"inputs"`
}

type Schema struct {
	Server ServerSchema `json:"server"`
}
