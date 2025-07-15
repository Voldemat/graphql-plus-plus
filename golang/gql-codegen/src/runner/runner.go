package runner

import "gql-codegen/src/config"

func Run(conf *config.GQLCodegenConfig) {
	for _, actor := range conf.Actors {
		actor(conf)
	}
}
