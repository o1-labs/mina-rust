/**
 * This configuration is used for fuzzing test environment.
 * Connects to fuzzing test nodes for stress testing.
 */

export default {
  production: false,
  globalConfig: {
    features: {
      dashboard: [],
      'block-production': ['overview', 'won-slots'],
    },
  },
  configs: [
    {
      name: 'Fuzzing Node',
      url: 'http://localhost:3085',
    },
  ],
};