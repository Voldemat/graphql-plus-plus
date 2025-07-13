/* eslint-disable max-lines */
import { dirname, resolve } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";
import { includeIgnoreFile } from "@eslint/compat";
import reactPlugin from 'eslint-plugin-react'

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const gitignorePath = resolve(__dirname, ".gitignore");

const compat = new FlatCompat({
    baseDirectory: __dirname,
});
const eslintConfig = [
    includeIgnoreFile(gitignorePath),
    {
        plugins: {
            react: reactPlugin
        },
        languageOptions: {
          parserOptions: {
            ecmaFeatures: {
              jsx: true,
            },
          },
        },
    },
    ...compat.config({
        rules: {
            "max-lines": [
                "error",
                100
            ],
            "max-len": [
                "error",
                80
            ],
            "quotes": [
                "error",
                "single"
            ],
            "array-callback-return": "error",
            "no-duplicate-imports": "error",
            "no-dupe-else-if": "error",
            "no-use-before-define": "error",
            "max-depth": [
                "error",
                3
            ],
            "no-shadow": "error",
            "no-extra-parens": [
                "error",
                "all"
            ],
            "indent": [
                "error",
                4
            ],
            "function-call-argument-newline": [
                "error",
                "consistent"
            ],
            "function-paren-newline": [
                "error",
                "consistent"
            ],
            "object-curly-spacing": [
                "error",
                "always"
            ],
            "no-console": "error",
        },
        "overrides": [
            {
                "files": [
                    "**/*.ts",
                    "**/*.tsx"
                ],
                "plugins": [
                    "@typescript-eslint",
                ],
                "extends": [
                    "plugin:@typescript-eslint/recommended"
                ],
                "parser": "@typescript-eslint/parser",
                "parserOptions": {
                    "project": [
                        "./tsconfig.json"
                    ]
                },
                "rules": {
                    "@typescript-eslint/no-unused-vars": "error",
                    "@typescript-eslint/no-deprecated": "error",
                    "@typescript-eslint/indent": "off",
                }
            }
        ]
    })
];

export default eslintConfig;
