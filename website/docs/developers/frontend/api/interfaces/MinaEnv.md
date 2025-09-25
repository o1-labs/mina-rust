[**Frontend TypeScript API**](../README.md)

***

[Frontend TypeScript API](../README.md) / MinaEnv

# Interface: MinaEnv

Defined in: [mina-env.type.ts:16](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L16)

Main environment configuration interface for the Mina Rust frontend.
This interface defines all possible configuration options that can be set
per environment (development, production, local, etc.).

To configure a frontend instance, modify the appropriate environment file:
- Development: src/environments/environment.ts
- Production: src/environments/environment.prod.ts
- Local: src/environments/environment.local.ts
- WebNode: src/environments/environment.webnodelocal.ts
- Producer: src/environments/environment.producer.ts
- Fuzzing: src/environments/environment.fuzzing.ts

## See

[https://github.com/o1-labs/mina-rust/tree/develop/frontend/src/environments](https://github.com/o1-labs/mina-rust/tree/develop/frontend/src/environments)

## Properties

### production

> **production**: `boolean`

Defined in: [mina-env.type.ts:18](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L18)

Whether this is a production build

***

### configs

> **configs**: [`MinaNode`](MinaNode.md)[]

Defined in: [mina-env.type.ts:21](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L21)

Array of Mina node configurations to connect to

***

### identifier?

> `optional` **identifier**: `string`

Defined in: [mina-env.type.ts:24](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L24)

Human-readable identifier for this environment (e.g., "Dev FE")

***

### hideToolbar?

> `optional` **hideToolbar**: `boolean`

Defined in: [mina-env.type.ts:27](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L27)

Hide the top toolbar in the UI

***

### hideNodeStats?

> `optional` **hideNodeStats**: `boolean`

Defined in: [mina-env.type.ts:30](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L30)

Hide node statistics display

***

### canAddNodes?

> `optional` **canAddNodes**: `boolean`

Defined in: [mina-env.type.ts:33](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L33)

Allow adding custom nodes through the UI

***

### showWebNodeLandingPage?

> `optional` **showWebNodeLandingPage**: `boolean`

Defined in: [mina-env.type.ts:36](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L36)

Show the WebNode landing page

***

### showLeaderboard?

> `optional` **showLeaderboard**: `boolean`

Defined in: [mina-env.type.ts:39](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L39)

Show the leaderboard/uptime tracking feature

***

### hidePeersPill?

> `optional` **hidePeersPill**: `boolean`

Defined in: [mina-env.type.ts:42](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L42)

Hide the peers pill in the status bar

***

### hideTxPill?

> `optional` **hideTxPill**: `boolean`

Defined in: [mina-env.type.ts:45](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L45)

Hide the transactions pill in the status bar

***

### sentry?

> `optional` **sentry**: `object`

Defined in: [mina-env.type.ts:48](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L48)

Sentry error tracking configuration

#### dsn

> **dsn**: `string`

Sentry Data Source Name for error reporting

#### tracingOrigins

> **tracingOrigins**: `string`[]

Origins to trace for performance monitoring

***

### globalConfig?

> `optional` **globalConfig**: `object`

Defined in: [mina-env.type.ts:56](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L56)

Global configuration shared across all nodes

#### features?

> `optional` **features**: [`FeaturesConfig`](../type-aliases/FeaturesConfig.md)

Feature flags configuration defining which UI sections are available

#### graphQL?

> `optional` **graphQL**: `string`

GraphQL endpoint URL for blockchain queries

#### firebase?

> `optional` **firebase**: `any`

Firebase configuration for leaderboard and hosting

#### heartbeats?

> `optional` **heartbeats**: `boolean`

Enable heartbeat/uptime tracking functionality
