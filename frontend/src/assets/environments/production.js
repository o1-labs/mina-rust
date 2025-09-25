/**
 * This configuration is used for production environment.
 * Connects to production network endpoints.
 */

export default {
  production: true,
  globalConfig: {
    features: {
      dashboard: [],
      nodes: ['overview', 'live', 'bootstrap'],
      state: ['actions'],
      snarks: ['scan-state', 'work-pool'],
      mempool: [],
      'block-production': ['won-slots'],
    },
  },
  configs: [
    {
      name: 'o1Labs Plain Node 1',
      url: 'http://mina-rust-plain-1.gcp.o1test.net/graphql',
    },
    {
      name: 'o1Labs Plain Node 2',
      url: 'http://mina-rust-plain-2.gcp.o1test.net/graphql',
    },
    {
      name: 'o1Labs Plain Node 3',
      url: 'http://mina-rust-plain-3.gcp.o1test.net/graphql',
    },
  ],
};
