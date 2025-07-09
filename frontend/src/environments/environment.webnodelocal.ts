import { MinaEnv } from '@shared/types/core/environment/mina-env.type';

export const environment: Readonly<MinaEnv> = {
  production: true,
  identifier: 'Web Node FE',
  canAddNodes: true,
  showWebNodeLandingPage: false,
  showLeaderboard: false,
  hidePeersPill: true,
  hideTxPill: true,
  globalConfig: {
    features: {
      dashboard: [],
      // nodes: ['overview', 'live', 'bootstrap'],
      state: ['actions'],
      // network: ['messages', 'connections', 'blocks', 'topology', 'node-dht', 'graph-overview', 'bootstrap-stats'],
      // snarks: ['scan-state', 'work-pool'],
      // resources: ['memory'],
      'block-production': ['won-slots'],
      mempool: [],
      benchmarks: ['wallets'],
      // fuzzing: [],
    },
    firebase: {
      apiKey: 'AIzaSyBZzFsHjIbQVbBP0N-KkUsEvHRVU_wwd7g',
      authDomain: 'webnode-gtm-test.firebaseapp.com',
      projectId: 'webnode-gtm-test',
      storageBucket: 'webnode-gtm-test.firebasestorage.app',
      messagingSenderId: '1016673359357',
      appId: '1:1016673359357:web:bbd2cbf3f031756aec7594',
      measurementId: 'G-ENDBL923XT',
    },
    heartbeats: true,
    graphQL: 'https://adonagy.com/graphql',
    // graphQL: 'https://api.minascan.io/node/devnet/v1/graphql',
    // graphQL: 'http://65.109.105.40:5000/graphql',
  },
  configs: [
    // {
    //   name: 'staging-devnet-bp-0',
    //   url: 'https://staging-devnet-openmina-bp-0.minaprotocol.network',
    // },
    // {
    //   name: 'staging-devnet-bp-1',
    //   url: 'https://staging-devnet-openmina-bp-1.minaprotocol.network',
    // },
    // {
    //   name: 'staging-devnet-bp-2',
    //   url: 'https://staging-devnet-openmina-bp-2.minaprotocol.network',
    // },
    // {
    //   name: 'staging-devnet-bp-3',
    //   url: 'https://staging-devnet-openmina-bp-3.minaprotocol.network',
    // },
    {
      name: 'Web Node',
      isWebNode: true,
    },
  ],
};

