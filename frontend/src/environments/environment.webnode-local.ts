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
      state: ['actions'],
      'block-production': ['won-slots'],
      mempool: [],
      benchmarks: ['wallets'],
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
    heartbeats: false,
  },
  configs: [
    {
      name: 'Web Node',
      isWebNode: true,
    },
  ],
};
