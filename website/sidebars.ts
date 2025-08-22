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
  // Sidebar for node operators - focus on operational guides
  nodeRunnersSidebar: [
    {
      type: 'category',
      label: 'Introduction',
      items: [
        'node-operators/getting-started',
        'node-operators/docker-usage',
        'node-operators/building-from-source',
      ],
    },
    {
      type: 'category',
      label: 'Node Types',
      items: [
        'node-operators/block-producer',
        'node-operators/archive-node',
      ],
    },
    {
      type: 'category',
      label: 'Operations',
      items: [
        'node-operators/local-demo',
        'node-operators/alpha-testing',
        'node-operators/network-configuration',
      ],
    },
    {
      type: 'category',
      label: 'Web Node',
      items: [
        'node-operators/webnode/local-webnode',
      ],
    },
    {
      type: 'category',
      label: 'Testing',
      items: [
        'node-operators/testing/overview',
      ],
    },
  ],

  // Sidebar for developers - focus on codebase and development
  developersSidebar: [
    {
      type: 'category',
      label: 'Introduction',
      items: [
        'developers/getting-started',
        'developers/updating-ocaml-node',
      ],
    },
    {
      type: 'category',
      label: 'Architecture',
      items: [
        'developers/why-rust',
        'developers/architecture',
      ],
    },
    {
      type: 'category',
      label: 'Docker',
      items: [
        'developers/docker-images',
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

  // Appendix sidebar - general reference material
  appendixSidebar: [
    {
      type: 'category',
      label: 'Installation References',
      items: [
        'appendix/docker-installation',
      ],
    },
    {
      type: 'category',
      label: 'Development Processes',
      items: [
        'appendix/release-process',
      ],
    },
  ],
};

export default sidebars;
