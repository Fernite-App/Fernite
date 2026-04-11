# Fernite Launcher

This is the Fernite Launcher monorepo — a Minecraft mod manager desktop application.

## Architecture

- **Monorepo tooling:** [Turborepo](https://turbo.build/) (`turbo.jsonc`) + [pnpm workspaces](https://pnpm.io/workspaces)
- **Frontend:** Vue 3 / Tauri, Tailwind CSS v3
- **Backend:** Rust (app-lib/theseus)
- **Indentation:** Use TAB everywhere, never spaces

### Apps (`apps/`)

| App            | Description                  |
| -------------- | ---------------------------- |
| `app`          | Desktop/app shell (Tauri)    |
| `app-frontend` | Desktop/app frontend (Vue 3) |

### Packages (`packages/`)

| Package            | Description                                              |
| ------------------ | -------------------------------------------------------- |
| `ui`               | Shared Vue component library (`@fernite/ui`)             |
| `assets`           | Styling and auto-generated icons (`@fernite/assets`)     |
| `api-client`       | API client for mod platform APIs (`@fernite/api-client`) |
| `app-lib`          | Core launcher library (theseus)                          |
| `utils`            | Shared utility functions                                 |
| `daedalus`         | Daedalus protocol (Minecraft auth)                       |
| `tooling-config`   | ESLint, Prettier, TypeScript configs                     |
| `ariadne`          | Analytics library                                        |
| `modrinth-log`     | Logging utilities                                        |
| `modrinth-maxmind` | MaxMind GeoIP                                            |
| `modrinth-util`    | General utilities                                        |
| `path-util`        | Path utilities                                           |
| `sqlx-tracing`     | SQLx query tracing                                       |

## Dev Commands

- **App:** `pnpm app:dev`
- **Build:** `pnpm app:build`

## Pre-PR Commands

Run from the **root** folder before opening a pull request:

- **App frontend:** `pnpm prepr:frontend:app`

## Code Guidelines

### Comments

- DO NOT use "heading" comments like: `=== Helper methods ===`.
- Use doc comments, but avoid inline comments unless ABSOLUTELY necessary for clarity. Code should aim to be self documenting!

## Bash Guidelines

### Output handling

- DO NOT pipe output through `head`, `tail`, `less`, or `more`
- NEVER use `| head -n X` or `| tail -n X` to truncate output
- IMPORTANT: Run commands directly without pipes when possible
- IMPORTANT: If you need to limit output, use command-specific flags

## Standards

Standards available at the `standards/` folder.
