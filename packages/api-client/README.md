# @fernite/api-client

[![TypeScript](https://img.shields.io/badge/TypeScript-5.0+-c78aff?style=for-the-badge)](https://www.typescriptlang.org/)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL%203.0-c78aff?style=for-the-badge)](LICENSE)

A flexible, type-safe API client for mod platform APIs. Works across Node.js, browsers, and Tauri with a feature system for authentication, retries, circuit breaking and other custom processing of requests and responses.

## Installation

```bash
pnpm add @fernite/api-client
# or
npm install @fernite/api-client
# or
yarn add @fernite/api-client
```

## Usage

### Tauri

```typescript
import { TauriModrinthClient, AuthFeature } from '@fernite/api-client'
import { getVersion } from '@tauri-apps/api/app'

const version = await getVersion()
const client = new TauriModrinthClient({
	userAgent: `fernite/theseus/${version}`,
	features: [new AuthFeature({ token: 'mrp_...' })],
})

const project = await client.request('/project/sodium', { api: 'labrinth', version: 2 })
```

### Overriding Base URLs

By default, the client uses Modrinth's production base URLs:

- `labrinthBaseUrl`: `https://api.modrinth.com/` (Labrinth API)
- `archonBaseUrl`: `https://archon.modrinth.com/` (Archon/Servers API)

You can override these for staging environments or custom instances:

```typescript
const client = new GenericModrinthClient({
	userAgent: 'my-app/1.0.0',
	labrinthBaseUrl: 'https://staging-api.modrinth.com/',
	archonBaseUrl: 'https://staging-archon.modrinth.com/',
	features: [new AuthFeature({ token: 'mrp_...' })],
})
```

## Features

### Authentication

Supports both static and dynamic tokens:

```typescript
// Static token
new AuthFeature({ token: 'mrp_...' })

// Dynamic token
new AuthFeature({
	token: async () => auth.value.token,
})
```

### Retry

Automatically retries failed requests with configurable backoff:

```typescript
new RetryFeature({
	maxAttempts: 3,
	backoffStrategy: 'exponential',
	initialDelay: 1000,
	maxDelay: 15000,
})
```

### Circuit Breaker

Prevents cascade failures by opening circuits after repeated failures:

```typescript
new CircuitBreakerFeature({
	maxFailures: 3,
	resetTimeout: 30000,
	failureStatusCodes: [500, 502, 503, 504],
})
```

## Documentation

This package is **self-documenting** through TypeScript types and JSDoc comments. Use your IDE's IntelliSense to explore available methods, classes, and configuration options.

For mod platform API endpoints and routes, refer to the [Modrinth API Documentation](https://docs.modrinth.com).

## Contributing

- Modules are available in the `modules/<api>/...` folders.
- When a module has different versions available, you should do it like so: `modules/labrinth/projects/v2.ts` etc.
- Types for a module's requests should be made available in `modules/<api>/module/types.ts` or `.../types/v2.ts`.
- You should expose these types in the `modules/types.ts` file.
- When creating a new module, add it to the `modules/index.ts`'s `MODULE_REGISTRY` for it to become available in the api client class.

Dont forget to run `pnpm fix` before committing.

## License

Licensed under GPL-3.0 - see the [LICENSE](LICENSE) file for details.
