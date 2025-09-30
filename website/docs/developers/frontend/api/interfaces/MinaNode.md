[**Frontend TypeScript API**](../README.md)

***

[Frontend TypeScript API](../README.md) / MinaNode

# Interface: MinaNode

Defined in: [mina-env.type.ts:72](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L72)

Configuration for a single Mina node connection.
Each node can have different endpoints and feature sets enabled.

## Properties

### name

> **name**: `string`

Defined in: [mina-env.type.ts:74](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L74)

Display name for this node (e.g., "Local rust node", "Producer-0")

***

### url?

> `optional` **url**: `string`

Defined in: [mina-env.type.ts:77](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L77)

Base URL for the node's API endpoint (e.g., "http://127.0.0.1:3000")

***

### memoryProfiler?

> `optional` **memoryProfiler**: `string`

Defined in: [mina-env.type.ts:80](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L80)

URL for memory profiling endpoint

***

### debugger?

> `optional` **debugger**: `string`

Defined in: [mina-env.type.ts:83](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L83)

URL for debugger interface

***

### features?

> `optional` **features**: [`FeaturesConfig`](../type-aliases/FeaturesConfig.md)

Defined in: [mina-env.type.ts:86](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L86)

Node-specific feature configuration (overrides globalConfig.features)

***

### isCustom?

> `optional` **isCustom**: `boolean`

Defined in: [mina-env.type.ts:89](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L89)

Whether this is a user-added custom node

***

### isWebNode?

> `optional` **isWebNode**: `boolean`

Defined in: [mina-env.type.ts:92](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L92)

Whether this node runs in the browser as a WebNode
