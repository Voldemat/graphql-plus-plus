package config

import (
	"gql-codegen/src/actors"
	"gql-codegen/src/schema"

	"go.uber.org/zap"
)

type GQLCodegenConfig struct {
	Logger *zap.Logger
	Schema *schema.Schema
	Actors []actors.Actor
}

func (c *GQLCodegenConfig) GetLogger() *zap.Logger {
	return c.Logger
}

func (c *GQLCodegenConfig) GetSchema() *schema.Schema {
	return c.Schema
}
