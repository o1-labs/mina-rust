/**
 * This configuration is used for production environment.
 * Connects to production network endpoints.
 */

export default {
  production: true,
  globalConfig: {
    features: {
      dashboard: [],
      'block-production': ['overview', 'won-slots'],
    },
  },
  sentry: {
    dsn: 'https://production-dsn-here@sentry.io/project-id',
    tracingOrigins: ['https://www.openmina.com'],
  },
  configs: [
    {
      name: 'Production Node',
      url: 'https://production-node.openmina.com',
    },
  ],
};
