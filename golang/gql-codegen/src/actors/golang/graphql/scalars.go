package graphql

import (
	"maps"

	"github.com/dave/jennifer/jen"
)

type ScalarStringSpec struct{}

func (s *ScalarStringSpec) OnObjectType(st *jen.Statement, nullable bool) {
    if nullable {
        st.Op("*")
    }
	st.String()
}
func (s *ScalarStringSpec) OnInputType(st *jen.Statement) {
	st.String()
}

type ScalarBooleanSpec struct{}

func (s *ScalarBooleanSpec) OnObjectType(st *jen.Statement, nullable bool) {
    if nullable {
        st.Op("*")
    }
	st.Bool()
}
func (s *ScalarBooleanSpec) OnInputType(st *jen.Statement) {
	st.Bool()
}

type ScalarIntSpec struct{}

func (s *ScalarIntSpec) OnObjectType(st *jen.Statement, nullable bool) {
    if nullable {
        st.Op("*")
    }
	st.Int32()
}
func (s *ScalarIntSpec) OnInputType(st *jen.Statement) {
	st.Int32()
}

type ScalarInt64Spec struct{}

func (s *ScalarInt64Spec) OnObjectType(st *jen.Statement, nullable bool) {
    if nullable {
        st.Op("*")
    }
	st.Int64()
}
func (s *ScalarInt64Spec) OnInputType(st *jen.Statement) {
	st.Int64()
}

type ScalarFloatSpec struct{}

func (s *ScalarFloatSpec) OnObjectType(st *jen.Statement, nullable bool) {
    if nullable {
        st.Op("*")
    }
	st.Float32()
}
func (s *ScalarFloatSpec) OnInputType(st *jen.Statement) {
	st.Float32()
}

type ScalarVoidSpec struct{}

func (s *ScalarVoidSpec) OnObjectType(st *jen.Statement, nullable bool) {
	st.Op("*").String()
}
func (s *ScalarVoidSpec) OnInputType(st *jen.Statement) {
	st.Op("*").String()
}

var DefaultScalarsMapping = map[string]ScalarSpec{
	"String":  &ScalarStringSpec{},
	"Boolean": &ScalarBooleanSpec{},
	"Int":     &ScalarIntSpec{},
	"Int64":   &ScalarInt64Spec{},
	"Float":   &ScalarFloatSpec{},
	"Void":    &ScalarVoidSpec{},
}

func MergeScalarsMappings(mappings ...map[string]ScalarSpec) map[string]ScalarSpec {
	var newMapping map[string]ScalarSpec = make(map[string]ScalarSpec)
	for _, m := range mappings {
		maps.Copy(newMapping, m)
	}
	return newMapping
}
