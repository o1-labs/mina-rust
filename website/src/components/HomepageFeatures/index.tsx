import type {ReactNode} from 'react';
import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: ReactNode;
};

const FeatureList: FeatureItem[] = [
    {
        title: 'Run a node anywhere',
        Svg: require('@site/static/img/Rust_programming_language_black_logo.svg').default,
        description: (
            <>
                In your browser. On your phone. Even in your Tesla.
            </>
        ),
    },
    {
        title: 'Why Rust? Safety and Stability',
        Svg: require('@site/static/img/Rust_programming_language_black_logo.svg').default,
        description: (
            <>
                Mina secures financial data — so we chose Rust for its memory safety,
                strong guarantees against concurrency bugs, and rock-solid
                reliability.
            </>
        ),
    },
    {
        title: 'Boosting Network Resilience',
        Svg: require('@site/static/img/Rust_programming_language_black_logo.svg').default,
        description: (
            <>
                Multiple node implementations mean bugs are caught faster and impact
                fewer nodes — reducing the risk to the whole network.
            </>
        ),
    },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): ReactNode {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
