package schema

import (
	"encoding/json"
	"os"

	"go.uber.org/zap"
)

func LoadSchemaFromFile(logger *zap.Logger, path string) *Schema {
	data, err := os.ReadFile(path)
	if err != nil {
		logger.Fatal("Couldn`t read schema file: ", zap.Error(err))
	}
	var schema Schema
	err = json.Unmarshal(data, &schema)
	if err != nil {
		logger.Fatal("Failed to parse json from schema file: ", zap.Error(err))
	}
	return &schema
}
