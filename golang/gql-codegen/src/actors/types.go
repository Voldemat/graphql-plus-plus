package actors

import (
	"go.uber.org/zap"
	"gql-codegen/src/schema"
)

type IActorConfig interface {
	GetLogger() *zap.Logger
	GetSchema() *schema.Schema
}

type Actor func(config IActorConfig)
