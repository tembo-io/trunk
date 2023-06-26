import Image from "next/image";
import Link from "next/link";
import styles from "./header.module.scss";
import React, { useState, useEffect, useRef } from "react";
import cx from "classnames";
const TrunkLogo = "/trunk_logo.png";
const SlackLogo = "/slack_logo.png";

export default function Header() {
  const [scrolled, setScrolled] = useState(false);
  const headerRef = useRef(null);
  useEffect(() => {
    function handleScroll() {
      if (window.scrollY >= 20) {
        setScrolled(true);
      } else {
        setScrolled(false);
      }
    }
    window.addEventListener("scroll", handleScroll);
    return () => {
      window.removeEventListener("scroll", handleScroll);
    };
  }, []);
  return (
    <header className={cx(styles.header, scrolled ? styles.headerScrolled : "")} ref={headerRef}>
      <Link href={"/"} shallow style={{ display: "flex", alignItems: "flex-start" }}>
        <Image width={29} height={29} style={{ marginRight: "10px" }} quality={20} priority alt="Trunk Logo" src={TrunkLogo}></Image>
        <h1 className={styles.title}>Trunk</h1>
      </Link>
      <div>
        <a className={styles.navLink} href={"https://github.com/tembo-io/trunk#installation"} target="_blank">
          Download
        </a>
        <a className={styles.navLink} href="https://github.com/tembo-io/trunk" target="_blank">
          Contribute
        </a>
        <a className={styles.navLink} href="https://tembo-io.github.io/trunk/" target="_blank">
          Docs
        </a>
        {/* TODO: Add blog */}
        <Link className={styles.navLink} href={"/"}>
          Blog
        </Link>
      </div>
      <div className={styles.ctaCont}>
        <span className={styles.ctaText}>Join Trunk on Slack</span>
        <Image width={16} height={16} style={{ marginLeft: "10px" }} quality={20} priority alt="Slack Logo" src={SlackLogo}></Image>
      </div>
    </header>
  );
}
