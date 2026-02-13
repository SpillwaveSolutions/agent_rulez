# monaco-types

[![github actions](https://github.com/remcohaszing/monaco-types/actions/workflows/ci.yaml/badge.svg)](https://github.com/remcohaszing/monaco-types/actions/workflows/ci.yaml)
[![npm version](https://img.shields.io/npm/v/monaco-types)](https://www.npmjs.com/package/monaco-types)
[![npm downloads](https://img.shields.io/npm/dm/monaco-types)](https://www.npmjs.com/package/monaco-types)

Users of Monaco editor may insert it into their webpage via multiple sources, for example from the
`monaco-editor` or `monaco-editor-core` package, from a CDN, or from a third party source.

This package provides type-only exports of Monaco editor from either `monaco-editor` or
`monaco-editor-core`, whatever is available.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Compatibility](#compatibility)
- [License](#license)

## Installation

```sh
npm install monaco-types
```

## Usage

Instead of importing from `monaco-editor` or `monaco-editor-core`, import all type definitions from
`monaco-types`. This will allow you to write type annotations:

```ts
import { MarkerSeverity } from 'monaco-types'

let severity: MarkerSeverity
```

But TypeScript will show an error when using the import as a value

```ts
import { MarkerSeverity } from 'monaco-types'

let severity: MarkerSeverity
```

Also your user will only have to install either `monaco-editor` or `monaco-editor-core` to get type
definitions, but you don’t have to enforce either on them.

In order to support this, your library needs to accept `MonacoEditor` as an argument instead of
importing. For example:

```ts
import { IDisposable, MonacoEditor } from 'monaco-types'

export function configureMyPlugin(monaco: MonacoEditor): IDisposable {
  // Use monaco here.
}
```

## Compatibility

This package follows the DefinitelyTyped
[support window](https://github.com/DefinitelyTyped/DefinitelyTyped#support-window).

## License

[MIT](LICENSE.md) © [Remco Haszing](https://github.com/remcohaszing)
