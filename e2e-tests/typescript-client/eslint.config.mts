import { dirname, resolve } from "path";
import { fileURLToPath } from "url";
import { FlatCompat } from "@eslint/eslintrc";
import { includeIgnoreFile } from "@eslint/compat";
import eslint from '@eslint/js';
import tseslint from 'typescript-eslint';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const gitignorePath = resolve(__dirname, ".gitignore");

const compat = new FlatCompat({
    baseDirectory: __dirname,
});
const eslintConfig = [
    includeIgnoreFile(gitignorePath),
    {
        ignores: ['shared/graphql/generated', 'shared/sql/generated']
    },
    {
        files: [
            "**/*.ts",
            "**/*.cts",
            "**.*.mts"
        ]
    },
    ...tseslint.config(
        eslint.configs.recommended,
        tseslint.configs.recommended,
    ),
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
        }
    })
];

export default eslintConfig;
