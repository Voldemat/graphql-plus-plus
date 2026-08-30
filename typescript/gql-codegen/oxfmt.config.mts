import { defineConfig } from 'oxfmt';

export default defineConfig({
    printWidth: 80,
    tabWidth: 4,
    singleQuote: true,
    ignorePatterns: ['tsconfig.json', 'package.json', 'package-lock.json']
});
