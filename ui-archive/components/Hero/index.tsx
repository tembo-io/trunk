import cx from "classnames";
import styles from "./Hero.module.scss";
import ExtensionSearchBox from "../ExtensionSearchBox";
import { Inter } from "next/font/google";

const inter = Inter({ subsets: ["latin"], weight: ["400", "500", "800"] });

export default function Hero() {
  return (
    <section>
      <h1 className={cx(inter.className, styles.title)}>Expand Your Postgres Capabilities</h1>
      <div className={styles.searchContainer}>
        <ExtensionSearchBox></ExtensionSearchBox>
      </div>
      <h2 className={cx(inter.className, styles.subtitle)}>The easiest way to publish and install PostgreSQL extensions. </h2>
      <div className={styles.linkRow}>
        <a href="https://tembo-io.github.io/trunk/" className={styles.linkButton}>
          <span className={cx(inter.className, styles.repoText)}>Get Started</span>
        </a>
        <a href="https://github.com/tembo-io/trunk" className={styles.linkButton}>
          <span className={cx(inter.className, styles.repoText)}>Contribute</span>
        </a>
      </div>
      <p className={cx(inter.className, styles.body)}>
        Trunk is an open-source package installer and registry for PostgreSQL extensions. Use the Trunk CLI to easily publish and install
        PostgreSQL extensions and their dependencies.
      </p>
    </section>
  );
}
