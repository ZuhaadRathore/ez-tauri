/**
 * ESLint configuration for TypeScript React project.
 *
 * Provides comprehensive linting rules for:
 * - TypeScript code quality and type safety
 * - React best practices and hooks rules
 * - Code formatting with Prettier integration
 * - Performance optimizations with React Refresh
 *
 * Excludes build artifacts and backend Rust code from linting.
 */

import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import react from 'eslint-plugin-react'
import tseslint from '@typescript-eslint/eslint-plugin'
import tsparser from '@typescript-eslint/parser'
import prettier from 'eslint-plugin-prettier'

export default [
  { ignores: ['dist', 'src-tauri'] }, // Exclude build artifacts and Rust backend
  {
    files: ['**/*.{ts,tsx}'], // Apply to TypeScript and React files
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
      parser: tsparser,
      parserOptions: {
        ecmaVersion: 'latest',
        ecmaFeatures: { jsx: true },
        sourceType: 'module',
      },
    },
    settings: { react: { version: '18.3' } },
    plugins: {
      react,
      'react-hooks': reactHooks,
      'react-refresh': reactRefresh,
      '@typescript-eslint': tseslint,
      prettier,
    },
    rules: {
      ...js.configs.recommended.rules,
      ...react.configs.recommended.rules,
      ...react.configs['jsx-runtime'].rules,
      ...reactHooks.configs.recommended.rules,
      ...tseslint.configs.recommended.rules,
      'react-refresh/only-export-components': [
        'warn',
        { allowConstantExport: true }, // Allow constant exports for better HMR
      ],
      'prettier/prettier': 'error',
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_' }, // Allow unused vars prefixed with underscore
      ],
      'react/prop-types': 'off', // TypeScript handles prop type checking
    },
  },
]
