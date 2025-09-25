[**Frontend TypeScript API**](../README.md)

***

[Frontend TypeScript API](../README.md) / FeaturesConfig

# Type Alias: FeaturesConfig

> **FeaturesConfig** = `Partial`\<\{ `dashboard`: `string`[]; `nodes`: `string`[]; `state`: `string`[]; `network`: `string`[]; `snarks`: `string`[]; `resources`: `string`[]; `block-production`: `string`[]; `mempool`: `string`[]; `benchmarks`: `string`[]; `fuzzing`: `string`[]; \}\>

Defined in: [mina-env.type.ts:108](https://github.com/o1-labs/mina-rust/blob/7580c8daad11ac442ed9418d7374d0829a14c197/frontend/src/app/shared/types/core/environment/mina-env.type.ts#L108)

Feature flags configuration that controls which UI sections and sub-features
are available. Each feature can have multiple sub-features enabled.

## Example

```typescript
features: {
  'dashboard': [],                    // Dashboard tab (no sub-features)
  'nodes': ['overview', 'live'],      // Nodes tab with overview and live sub-tabs
  'network': ['messages', 'blocks']   // Network tab with specific sub-sections
}
```
