/**
 * This configuration is used for local development environment.
 * Connects to a local Mina node running on localhost.
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
      name: 'Local Node',
      url: 'http://localhost:3085',
    },
  ],
};
