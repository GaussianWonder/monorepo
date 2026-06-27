# Monorepo

Opinionated monorepo powered by [moon](https://moonrepo.dev/) and [proto](https://moonrepo.dev/proto) with code generation templates, unified build pipelines, and dependency version management.

- **Primary focus: TypeScript** (Node 26, TypeScript 6)
- **Secondary support: Go** (Go 1.26)

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
5. Prepare tooling
   - Go: `moon run root:prepare-go`
6. Sync configs: `moon sync config-schemas`
7. Generate your first project:
   - TypeScript: `moon generate node-app` or `moon generate node-package`
   - Go: `moon generate go-app` or `moon generate go-package`
8. Sync apps and packages: `moon sync`

## Project Structure

```
apps/          # Applications (entry points, deployable artifacts)
packages/      # Shared libraries (consumed by apps and other packages)
infra/         # Infrastructure tooling
tests/         # Test apps, toy projects, and POCs
templates/     # Moon code generation templates
.moon/         # Moon workspace and task configuration
```

## TypeScript Tooling

### Build Pipeline

All TypeScript projects inherit shared tasks from `.moon/tasks/node.yml`. Source files live in `src/`, build output goes to `dist/`.

| Task         | Command                                    | Purpose                                          |
| ------------ | ------------------------------------------ | ------------------------------------------------ |
| `dev`        | `tsx src/index.ts`                         | Run from source with typechecking                |
| `typecheck`  | `tsc --build`                              | Type checking with project references            |
| `build`      | `tsdown --sourcemap`                       | Build with sourcemaps and type declarations      |
| `bundle`     | `tsdown --sourcemap --minify --no-dts`     | Minified production bundle, no type declarations |
| `start`      | `node --enable-source-maps dist/index.mjs` | Run the `build` output                           |
| `start-prod` | `node --enable-source-maps dist/index.mjs` | Run the `bundle` output                          |
| `check`      | `biome check --write`                      | Lint and format                                  |

`start` and `start-prod` automatically build workspace dependencies (`^:build`) before running.

### Apps vs Packages

**Packages** produce importable libraries. Their `build` output includes type declarations (`.d.mts`) and externalizes `dependencies` by default. Packages exclude the `bundle` task since they are not standalone executables.

**Apps** produce runnable artifacts. `build` is the same as packages. `bundle` creates a single-file minified output with all dependencies inlined (`alwaysBundle: ["**"]` in `package.json`).

### Bundling and Native Packages

Apps bundle all dependencies by default via the `tsdown` config in `package.json`:

```json
{
  "tsdown": {
    "deps": {
      "alwaysBundle": ["**"]
    }
  }
}
```

For native packages that cannot be bundled (e.g. `better-sqlite3`), exclude them per-app:

```json
{
  "dependencies": {
    "better-sqlite3": "^11.0.0"
  },
  "tsdown": {
    "deps": {
      "alwaysBundle": ["**"],
      "neverBundle": ["better-sqlite3"]
    }
  }
}
```

Externalized packages must be declared in the app's `dependencies` so they are available in `node_modules` at runtime.

Packages do not need any `tsdown` config. Dependencies listed in `package.json` `dependencies` are externalized by default.

### TypeScript Configuration

The tsconfig chain is:

```
tsconfig-moon/tsconfig.projects.json   (base: strict, composite, declarations)
  -> tsconfig.options.json             (overrides: es2025, nodenext, types: ["node"])
    -> <project>/tsconfig.json         (project-specific: rootDir, outDir)
```

`@types/node` is installed at the root and made available to all projects via `types: ["node"]` in `tsconfig.options.json`. Projects that need different types (e.g. React, Next.js) can override `types` in their own `tsconfig.json` — the field fully replaces the parent.

### Dependency Version Management

[syncpack](https://syncpack.dev/) enforces consistent dependency versions across all `package.json` files.

| Task               | Command                           | Purpose                           |
| ------------------ | --------------------------------- | --------------------------------- |
| `syncpack-check`   | `syncpack lint`                   | Check for version mismatches (CI) |
| `syncpack-fix`     | `syncpack fix`                    | Auto-fix version mismatches       |
| `syncpack-update`  | `syncpack update --target minor`  | Safe minor/patch updates          |
| `syncpack-upgrade` | `syncpack update --target latest` | Full upgrade including majors     |

Run from root: `moon run root:syncpack-check`

Configuration (`.syncpackrc.json`):

- Enforces `^` semver ranges across all packages
- Uses `workspace:*` protocol for local package references
- Sources auto-discovered from `pnpm-workspace.yaml`

## Project Templates

### TypeScript

| Template       | Command                      | Creates                                                     |
| -------------- | ---------------------------- | ----------------------------------------------------------- |
| `node-app`     | `moon generate node-app`     | Application with bundling, start tasks, and `tsdown` config |
| `node-package` | `moon generate node-package` | Library with type declarations and package exports          |

### Go

| Template     | Command                    | Creates                           |
| ------------ | -------------------------- | --------------------------------- |
| `go-app`     | `moon generate go-app`     | Go application with module config |
| `go-package` | `moon generate go-package` | Go library with module config     |

After generating a Go project, add it to the workspace:

```bash
go work use apps/yourgoappname
# or
go work use packages/yourgopackagename
```

## Prepare Tasks

Root-level tasks that install language-specific tooling not managed by proto. Run once after cloning, or in CI before build steps.

| Task         | Command                    | Installs                                                     |
| ------------ | -------------------------- | ------------------------------------------------------------ |
| `prepare-go` | `moon run root:prepare-go` | [staticcheck](https://staticcheck.dev/) (Go static analysis) |

## Customization

### Before Using This Template

1. **Author Information**: Update `author.default` in all `templates/*/template.yml` files
2. **License**: Update `license.default` in template files if needed
3. **LICENSE File**: Replace root and template LICENSE files with your own
