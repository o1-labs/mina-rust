import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)

const config: Config = {
  title: 'OpenMina Documentation',
  tagline: 'Rust implementation of the Mina Protocol - lightweight blockchain using zero-knowledge proofs',
  favicon: 'img/favicon.ico',

  // Future flags, see https://docusaurus.io/docs/api/docusaurus-config#future
  future: {
    v4: true, // Improve compatibility with the upcoming Docusaurus v4
  },

  // Set the production url of your site here
  url: 'https://o1-labs.github.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/openmina/',

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'o1-labs', // Usually your GitHub org/user name.
  projectName: 'openmina', // Usually your repo name.

  onBrokenLinks: 'warn', // Temporarily set to warn while we fix broken links
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      {
        docs: {
          sidebarPath: './sidebars.ts',
          // Enable versioning
          includeCurrentVersion: true,
          lastVersion: 'current',
          versions: {
            current: {
              label: 'develop',
            },
          },
          // Enable edit links to GitHub
          editUrl: 'https://github.com/o1-labs/openmina/tree/develop/docs/',
        },
        blog: false, // Disable blog for technical documentation
        theme: {
          customCss: './src/css/custom.css',
        },
      } satisfies Preset.Options,
    ],
  ],


  themeConfig: {
    // Replace with your project's social card
    image: 'img/docusaurus-social-card.jpg',

    // SEO improvements and metadata
    metadata: [
      {name: 'keywords', content: 'OpenMina, Mina Protocol, Rust, blockchain, zero-knowledge proofs, zkProofs, cryptocurrency, decentralized'},
      {name: 'description', content: 'OpenMina is a Rust implementation of the Mina Protocol - a lightweight blockchain using zero-knowledge proofs. Learn how to run nodes, develop applications, and understand the protocol.'},
      {name: 'author', content: 'o1Labs'},
      {name: 'robots', content: 'index,follow'},
      {name: 'googlebot', content: 'index,follow'},
      {property: 'og:type', content: 'website'},
      {property: 'og:title', content: 'OpenMina Documentation'},
      {property: 'og:description', content: 'OpenMina is a Rust implementation of the Mina Protocol - a lightweight blockchain using zero-knowledge proofs.'},
      {property: 'og:image', content: 'https://o1-labs.github.io/openmina/img/docusaurus-social-card.jpg'},
      {property: 'twitter:card', content: 'summary_large_image'},
      {property: 'twitter:title', content: 'OpenMina Documentation'},
      {property: 'twitter:description', content: 'OpenMina is a Rust implementation of the Mina Protocol - a lightweight blockchain using zero-knowledge proofs.'},
      {property: 'twitter:image', content: 'https://o1-labs.github.io/openmina/img/docusaurus-social-card.jpg'},
    ],
    navbar: {
      title: 'OpenMina',
      logo: {
        alt: 'OpenMina Logo',
        src: 'img/logo.svg',
      },
      items: [
        {
          type: 'docSidebar',
          sidebarId: 'nodeRunnersSidebar',
          position: 'left',
          label: 'Node Runners',
        },
        {
          type: 'docSidebar',
          sidebarId: 'developersSidebar',
          position: 'left',
          label: 'Developers',
        },
        {
          type: 'docSidebar',
          sidebarId: 'researchersSidebar',
          position: 'left',
          label: 'Researchers',
        },
        {
          href: '/api-docs/',
          label: 'API Docs',
          position: 'left',
        },
        {
          type: 'docsVersionDropdown',
          position: 'right',
          dropdownActiveClassDisabled: true,
        },
        {
          href: 'https://github.com/o1-labs/openmina',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Documentation',
          items: [
            {
              label: 'Node Runners',
              to: '/docs/node-runners/getting-started',
            },
            {
              label: 'Developers',
              to: '/docs/developers/architecture',
            },
            {
              label: 'Researchers',
              to: '/docs/researchers/protocol',
            },
            {
              label: 'API Documentation',
              to: '/api-docs/',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'GitHub',
              href: 'https://github.com/o1-labs/openmina',
            },
            {
              label: 'Issues',
              href: 'https://github.com/o1-labs/openmina/issues',
            },
            {
              label: 'Discussions',
              href: 'https://github.com/o1-labs/openmina/discussions',
            },
            {
              label: 'Twitter',
              href: 'https://x.com/o1_labs',
            },
            {
              label: 'Discord',
              href: 'https://discord.gg/minaprotocol',
            },
            {
              label: 'LinkedIn',
              href: 'https://www.linkedin.com/company/o1labs/',
            },
          ],
        },
        {
          title: 'Resources',
          items: [
            {
              label: 'Mina Protocol',
              href: 'https://minaprotocol.com/',
            },
            {
              label: 'Mina Protocol Docs',
              href: 'https://docs.minaprotocol.com/',
            },
            {
              label: 'o1Labs',
              href: 'https://www.o1labs.org/',
            },
            {
              label: 'o1Labs GitHub',
              href: 'https://github.com/o1-labs',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} o1Labs. Built with Docusaurus.`,
    },
    prism: {
      theme: prismThemes.github,
      darkTheme: prismThemes.dracula,
      additionalLanguages: ['rust', 'toml', 'bash', 'docker', 'yaml', 'json'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
