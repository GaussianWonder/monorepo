# Monorepo

Opinionated monorepo powered by [moon](https://moonrepo.dev/) and [proto](https://moonrepo.dev/proto) with code generation templates, unified build pipelines, and dependency version management.

- **Primary focus: TypeScript** (Node 26, TypeScript 6)
- **Secondary support: Go** (Go 1.26), **Rust** (Rust 1.96)

- [Monorepo](#monorepo)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Quick Start](#quick-start)
  - [Project Structure](#project-structure)
  - [TypeScript Tooling](#typescript-tooling)
    - [Build Pipeline](#build-pipeline)
    - [Switching Runtime (Node or Bun)](#switching-runtime)
    - [Apps vs Packages](#apps-vs-packages)
    - [Bundling and Native Packages](#bundling-and-native-packages)
      - [Containerization](#containerization)
    - [TypeScript Configuration](#typescript-configuration)
    - [Dependency Version Management](#dependency-version-management)
  - [Project Templates](#project-templates)
    - [TypeScript](#typescript)
    - [Go](#go)
    - [Rust](#rust)
      - [Rust Tasks](#rust-tasks)
  - [Customization](#customization)
    - [Before Using This Template](#before-using-this-template)

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
5. Sync configs: `moon sync config-schemas`
6. Generate your first project:
   - TypeScript: `moon generate node-app` or `moon generate node-package`
   - Go: `moon generate go-app` or `moon generate go-package`
   - Rust: `moon generate rust-app` or `moon generate rust-package`
7. Sync apps and packages: `moon sync`

## Project Structure

```text
apps/          # Applications (entry points, deployable artifacts)
packages/      # Shared libraries (consumed by apps and other packages)
infra/         # Infrastructure tooling
tests/         # Test apps, toy projects, and POCs
templates/     # Moon code generation templates
.moon/         # Moon workspace and task configuration
```

## TypeScript Tooling

### Build Pipeline

TypeScript tasks are split across three files to support swappable runtimes:

- **`javascript.yml`**: shared build tooling (tsdown, tsc, biome) inherited by all JS/TS projects
- **`node.yml`**: runtime tasks (`dev`, `start`, `start-prod`) via `node`/`tsx`, tagged `node_runtime`
- **`bun.yml`**: same runtime tasks via `bun`, tagged `bun_runtime`

Source files live in `src/`, build output goes to `dist/`.

| Task         | Command                                      | Source                 | Purpose                                          |
| ------------ | -------------------------------------------- | ---------------------- | ------------------------------------------------ |
| `typecheck`  | `tsc --build`                                | `javascript.yml`       | Type checking with project references            |
| `build`      | `tsdown --sourcemap`                         | `javascript.yml`       | Build with sourcemaps and type declarations      |
| `bundle`     | `tsdown --sourcemap --minify --no-dts`       | `javascript.yml`       | Minified production bundle, no type declarations |
| `check`      | `biome check --write`                        | `javascript.yml`       | Lint and format                                  |
| `dev`        | `tsx src/index.ts` / `bun src/index.ts`      | `node.yml` / `bun.yml` | Run from source with typechecking                |
| `start`      | `node dist/index.mjs` / `bun dist/index.mjs` | `node.yml` / `bun.yml` | Run the `build` output                           |
| `start-prod` | `node dist/index.mjs` / `bun dist/index.mjs` | `node.yml` / `bun.yml` | Run the `bundle` output                          |

`start` and `start-prod` automatically build workspace dependencies (`^:build`) before running.

### Switching Runtime

Projects default to Node via the `node_runtime` tag in templates. To switch to Bun, change the tag in the project's `moon.yml`:

```yaml
tags:
  - bun_runtime # instead of node_runtime
```

Build tooling (tsdown, tsc, biome) is shared, only the runtime commands (`dev`, `start`, `start-prod`) change.

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

#### Containerization

When deploying a bundled app, only externalized (native) packages need `node_modules` at runtime. To keep images small:

1. Externalize native packages via `neverBundle`
2. Create a `production.package.json` listing only externalized packages
3. `pnpm install --prod` from that file in your Dockerfile

### TypeScript Configuration

The tsconfig chain is:

```text
tsconfig-moon/tsconfig.projects.json   (base: strict, composite, declarations)
  -> tsconfig.options.json             (overrides: es2025, nodenext, types: ["node"])
    -> <project>/tsconfig.json         (project-specific: rootDir, outDir)
```

`@types/node` is installed at the root and made available to all projects via `types: ["node"]` in `tsconfig.options.json`. Projects that need different types (e.g. React, Next.js) can override `types` in their own `tsconfig.json`, the field fully replaces the parent.

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

### Rust

| Template       | Command                      | Creates                               |
| -------------- | ---------------------------- | ------------------------------------- |
| `rust-app`     | `moon generate rust-app`     | Binary crate with `main.rs`           |
| `rust-package` | `moon generate rust-package` | Library crate with `lib.rs` and tests |

After generating a Rust project, add it to the `members` list in the root `Cargo.toml`:

```toml
[workspace]
members = [
  "apps/my_app",
  "packages/my_lib",
]
```

Cargo workspace globs are not used because not all directories under `apps/` or `packages/` contain a `Cargo.toml` — members must be registered manually.

Use underscores for crate names (`my_app`, not `my-app`).

To use a local package from an app, add it to `apps/my_app/Cargo.toml`:

```toml
[dependencies]
my_lib = { path = "../../packages/my_lib" }
```

#### Rust Tasks

| Task        | Command               | Purpose                   |
| ----------- | --------------------- | ------------------------- |
| `dev`       | `cargo run`           | Run from source (debug)   |
| `build`     | `cargo build`         | Debug build               |
| `start`     | `cargo run --release` | Optimized release run     |
| `typecheck` | `cargo check`         | Fast type/borrow checking |
| `lint`      | `cargo clippy`        | Linting                   |
| `format`    | `cargo fmt --check`   | Format checking           |
| `check`     | clippy + fmt          | Combined lint and format  |
| `test`      | `cargo test`          | Run tests                 |

No `bundle` task. `cargo build --release` already produces a single statically linked binary. Packages exclude `build`, `dev`, and `start`.

## Customization

### Before Using This Template

1. **Author Information**: Update `author.default` in all `templates/*/template.yml` files
2. **License**: Update `license.default` in template files if needed
3. **LICENSE File**: Replace root and template LICENSE files with your own
