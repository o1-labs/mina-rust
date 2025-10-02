import type {ReactNode} from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';
import MobileThemeToggle from '@site/src/components/MobileThemeToggle';

import styles from './index.module.css';

interface QuickNavItem {
  title: string;
  description: string;
  to: string;
  icon: ReactNode;
}

const quickNavItems: QuickNavItem[] = [
  {
    title: 'Node Operators',
    description: 'Run and maintain Mina Rust Node instances',
    to: '/docs/node-operators/getting-started',
    icon: (
      <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
        <path d="M12 2L2 7V10C2 16 6 20.9 12 22C18 20.9 22 16 22 10V7L12 2Z"/>
      </svg>
    ),
  },
  {
    title: 'Developers',
    description: 'Build applications and contribute code',
    to: '/docs/developers/getting-started',
    icon: (
      <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
        <path d="M9.4 16.6L4.8 12L9.4 7.4L8 6L2 12L8 18L9.4 16.6ZM14.6 16.6L19.2 12L14.6 7.4L16 6L22 12L16 18L14.6 16.6Z"/>
      </svg>
    ),
  },
  {
    title: 'API Docs',
    description: 'Comprehensive API reference',
    to: 'https://o1-labs.github.io/mina-rust/api-docs/',
    icon: (
      <svg viewBox="0 0 24 24" width="24" height="24" fill="currentColor">
        <path d="M14,2H6A2,2 0 0,0 4,4V20A2,2 0 0,0 6,22H18A2,2 0 0,0 20,20V8L14,2M18,20H6V4H13V9H18V20Z"/>
      </svg>
    ),
  },
];

function QuickNavCard({title, description, to, icon}: QuickNavItem) {
  const isExternal = to.startsWith('http');

  return (
    <Link
      to={to}
      className={styles.quickNavCard}
      {...(isExternal && { target: '_blank', rel: 'noopener noreferrer' })}
    >
      <div className={styles.cardIcon}>
        {icon}
      </div>
      <div className={styles.cardContent}>
        <h3 className={styles.cardTitle}>{title}</h3>
        <p className={styles.cardDescription}>{description}</p>
      </div>
      <div className={styles.cardArrow}>
        <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
          <path d="M8.59 16.59L13.17 12L8.59 7.41L10 6L16 12L10 18L8.59 16.59Z"/>
        </svg>
      </div>
    </Link>
  );
}

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={styles.hero}>
      <div className={styles.heroBackground}>
        <svg className={styles.heroBackgroundSvg} viewBox="0 0 800 600" preserveAspectRatio="xMidYMid slice">
          <defs>
            <linearGradient id="heroGradient" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stopColor="var(--accent)" stopOpacity="0.1" />
              <stop offset="100%" stopColor="var(--accent-2)" stopOpacity="0.05" />
            </linearGradient>
          </defs>
          <rect width="100%" height="100%" fill="url(#heroGradient)" />
          <g opacity="0.3">
            <circle cx="200" cy="150" r="80" fill="none" stroke="var(--accent)" strokeWidth="1" strokeOpacity="0.2" />
            <circle cx="600" cy="300" r="120" fill="none" stroke="var(--accent-2)" strokeWidth="1" strokeOpacity="0.15" />
            <circle cx="150" cy="400" r="60" fill="none" stroke="var(--accent-3)" strokeWidth="1" strokeOpacity="0.1" />
          </g>
        </svg>
      </div>
      <div className="container">
        <div className={styles.heroContent}>
          <Heading as="h1" className={styles.heroTitle}>
            {siteConfig.title}
          </Heading>
          <p className={styles.heroSubtitle}>
            Rust implementation of the Mina Protocol — lightweight blockchain using zero knowledge proofs.
          </p>
          <div className={styles.heroButtons}>
            <Link className={styles.btnPrimary} to="/docs/node-operators/getting-started">
              Get Started
              <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                <path d="M8.59 16.59L13.17 12L8.59 7.41L10 6L16 12L10 18L8.59 16.59Z"/>
              </svg>
            </Link>
            <Link className={styles.btnSecondary} to="/docs/node-operators/join-devnet">
              Join Devnet
              <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
                <path d="M8.59 16.59L13.17 12L8.59 7.41L10 6L16 12L10 18L8.59 16.59Z"/>
              </svg>
            </Link>
          </div>
        </div>
      </div>
    </header>
  );
}

function QuickNavSection() {
  return (
    <section className={styles.quickNav}>
      <div className="container">
        <div className={styles.quickNavGrid}>
          {quickNavItems.map((item, idx) => (
            <QuickNavCard key={idx} {...item} />
          ))}
        </div>
      </div>
    </section>
  );
}

function ProjectBlurb() {
  return (
    <section className={styles.projectBlurb}>
      <div className="container">
        <div className={styles.blurbContent}>
          <p>
            The Mina Rust Node is an open-source Rust implementation of the{' '}
            <Link href="https://minaprotocol.com/" target="_blank" rel="noopener noreferrer">
              Mina Protocol
            </Link>
            , originally written in OCaml. Built in Rust for enhanced performance and reliability,
            it provides a full node implementation with advanced debugging capabilities and a modular
            architecture for researchers and developers. It also includes the Mina Web Node,
            a browser-based implementation that allows users to run a full Mina node
            directly in their web browser without any installation. The Mina Web Node can even
            participate in consensus and produce blocks by simply leaving a browser tab open,
            earning rewards for successful block production.
          </p>
          <div className={styles.githubLink}>
            <Link
              href="https://github.com/o1-labs/mina-rust"
              target="_blank"
              rel="noopener noreferrer"
              className={styles.btnSecondary}
            >
              <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor">
                <path d="M12,2A10,10 0 0,0 2,12C2,16.42 4.87,20.17 8.84,21.5C9.34,21.58 9.5,21.27 9.5,21C9.5,20.77 9.5,20.14 9.5,19.31C6.73,19.91 6.14,17.97 6.14,17.97C5.68,16.81 5.03,16.5 5.03,16.5C4.12,15.88 5.1,15.9 5.1,15.9C6.1,15.97 6.63,16.93 6.63,16.93C7.5,18.45 8.97,18 9.54,17.76C9.63,17.11 9.89,16.67 10.17,16.42C7.95,16.17 5.62,15.31 5.62,11.5C5.62,10.39 6,9.5 6.65,8.79C6.55,8.54 6.2,7.5 6.75,6.15C6.75,6.15 7.59,5.88 9.5,7.17C10.29,6.95 11.15,6.84 12,6.84C12.85,6.84 13.71,6.95 14.5,7.17C16.41,5.88 17.25,6.15 17.25,6.15C17.8,7.5 17.45,8.54 17.35,8.79C18,9.5 18.38,10.39 18.38,11.5C18.38,15.32 16.04,16.16 13.81,16.41C14.17,16.72 14.5,17.33 14.5,18.26C14.5,19.6 14.5,20.68 14.5,21C14.5,21.27 14.66,21.59 15.17,21.5C19.14,20.16 22,16.42 22,12A10,10 0 0,0 12,2Z"/>
              </svg>
              View on GitHub
            </Link>
          </div>
        </div>
      </div>
    </section>
  );
}

export default function Home(): ReactNode {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title}`}
      description={siteConfig.tagline}
    >
      <MobileThemeToggle />
      <HomepageHeader />
      <QuickNavSection />
      <ProjectBlurb />
    </Layout>
  );
}
