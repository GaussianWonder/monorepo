# Typescript Moon

Opinionated template to kick-start moon projects with TypeScript, moon workspace management, and code generation templates.

> See [moonrepo](https://moonrepo.dev/)

## Getting Started

### Prerequisites

- [Moon CLI](https://moonrepo.dev/docs/install) installed
- [Proto](https://moonrepo.dev/docs/proto/install) installed

### Quick Start

1. Clone this repository
2. **IMPORTANT**: Update internal template variables (see [Customization](#customization))
3. **IMPORTANT**: Update or replace the LICENSE file with your own
4. Install dependencies
    - `proto install`
    - `pnpm install`
5. Generate your first project: `moon generate node-app` or `moon generate node-package`
6. Sync apps and packages: `moon sync`

## Project Templates

This repository includes two code generation templates:

### `node-app` - TypeScript Application

Creates a Node.js application with:

- TypeScript configuration with project references
- Moon workspace integration
- esbuild bundling setup
- Development server with tsx
- Type checking tasks

**Usage:**

```bash
moon generate node-app
```

### `node-package` - TypeScript Package/Library

Creates a TypeScript package/library with:

- TypeScript configuration optimized for libraries
- Moon workspace integration
- Generated files support
- JSDoc documentation structure

**Usage:**

```bash
moon generate node-package
```

## Customization

### Before Using This Template

**⚠️ IMPORTANT**: Update these internal variables before usage:

1. **Author Information**: Update the default author in both template files:
   - `templates/node-app/template.yml`
   - `templates/node-package/template.yml`

   Change the `author.default` value from:

   ```yaml
   author:
     type: "string"
     default: "Virghileanu Teodor <teo.virghi@gmail.com> (https://github.com/GaussianWonder)"
     internal: true
   ```

2. **License**: Update the default license if needed:

   ```yaml
   license:
     type: "string"
     default: "ISC"  # Change this if you prefer a different license
     internal: true
   ```

3. **LICENSE File**: Replace the root LICENSE file and template LICENSE files with your own license

### Template Variables

When generating projects, you'll be prompted for:

- **name**: Project/package name (required)
- **description**: Project/package description (required)

These variables are automatically set (no prompting):

- **author**: Uses the default from template.yml
- **license**: Uses the default from template.yml
