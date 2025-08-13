import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  // Sidebar for node runners - focus on operational guides
  nodeRunnersSidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'node-runners/getting-started',
        'node-runners/docker-installation',
        'node-runners/building-from-source',
      ],
    },
    {
      type: 'category',
      label: 'Node Types',
      items: [
        'node-runners/block-producer',
        'node-runners/archive-node',
      ],
    },
    {
      type: 'category',
      label: 'Operations',
      items: [
        'node-runners/local-demo',
        'node-runners/alpha-testing',
        'node-runners/network-configuration',
      ],
    },
    {
      type: 'category',
      label: 'Web Node',
      items: [
        'node-runners/webnode/local-webnode',
      ],
    },
    {
      type: 'category',
      label: 'Testing',
      items: [
        'node-runners/testing/overview',
      ],
    },
  ],

  // Sidebar for developers - focus on codebase and development
  developersSidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'developers/getting-started',
        'developers/updating-ocaml-node',
      ],
    },
    {
      type: 'category',
      label: 'Architecture',
      items: [
        'developers/why-openmina',
        'developers/architecture',
      ],
    },
    {
      type: 'category',
      label: 'P2P Networking',
      items: [
        'developers/p2p-networking',
        'developers/webrtc',
        'developers/libp2p',
      ],
    },
  ],

  // Sidebar for researchers - focus on protocol and cryptography
  researchersSidebar: [
    {
      type: 'category',
      label: 'Protocol',
      items: [
        'researchers/protocol',
        'researchers/scan-state',
      ],
    },
    {
      type: 'category',
      label: 'Cryptography',
      items: [
        'researchers/snark-work',
      ],
    },
  ],
};

export default sidebars;
