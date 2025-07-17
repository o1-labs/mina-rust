const { defineConfig } = require('cypress')
import { resolve } from 'path';

module.exports = defineConfig({
  chromeWebSecurity: false,
  viewportWidth: 1920,
  viewportHeight: 1080,
  component: {
    devServer: {
      framework: 'angular',
      bundler: 'vite',
      viteConfig: {
        resolve: {
          alias: {
            '../public': resolve(__dirname, './public'),
          }
        }
      }
    },
    specPattern: '**/*.cy.ts',
  },
  resolve: {
    alias: {
      // Map ../public to the actual public folder
      '../public': resolve(__dirname, './public'),
    },
  },
  server: {
    fs: {
      // Allow serving files from the public directory
      allow: ['.'],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, 'index.html'),
      },
    },
  },
  numTestsKeptInMemory: 20,
  experimentalMemoryManagement: true,
  defaultCommandTimeout: 10000,
  video: false,
  e2e: {
    baseUrl: 'http://localhost:4200',
    setupNodeEvents(on, config) {
      return require('./cypress/plugins/index.js')(on, config);
    }
  },
  include: [
    '**/*.ts',
    '**/*.js',
    'node_modules/rxjs/**/*.js'
  ]
});
