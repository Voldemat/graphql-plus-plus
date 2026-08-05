import { defineConfig } from 'oxlint';

export default defineConfig({
  ignorePatterns: ['.gitignore'],
  rules: {
    'max-lines': ['error', 100],
    'array-callback-return': 'error',
    'no-duplicate-imports': 'error',
    'no-dupe-else-if': 'error',
    'no-use-before-define': 'error',
    'max-depth': ['error', 3],
    'no-shadow': 'error',
    'no-console': 'error',
  },
});
