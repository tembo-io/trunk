import Image from "next/image";
import Link from "next/link";
import styles from "./header.module.scss";
import React, { useState, useEffect, useRef } from "react";
import cx from "classnames";
const TrunkLogo = "/TrunkLogo.png";
const SlackLogo = "/slack_logo.png";
import Search from "../Search";

export default function Header({ white = false, search = false, extensions = [] }) {
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
    <header
      style={white ? { backgroundColor: "white" } : {}}
      className={cx(styles.header, scrolled ? styles.headerScrolled : "")}
      ref={headerRef}
    >
      <div style={{ display: "flex" }}>
        <Link href={"/"} shallow style={{ display: "flex", alignItems: "flex-start" }}>
          <Image
            width={108}
            height={38}
            style={{ marginRight: "10px", minWidth: "108px", minHeight: "38px" }}
            quality={90}
            priority
            alt="Trunk Logo"
            src={TrunkLogo}
          ></Image>
        </Link>
        {search && (
          <div style={{ marginLeft: "20px" }}>
            <Search extensions={extensions}></Search>
          </div>
        )}
      </div>
      <div style={search ? { marginLeft: "auto" } : {}}>
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
      <a
        target="_blank"
        href="https://trunk-crew.slack.com/join/shared_invite/zt-1w6kv2plz-vJlgjXzTSALlvjJyU8lhcQ#/shared-invite/email"
        className={styles.ctaCont}
      >
        <span className={styles.ctaText}>Join Trunk on Slack</span>
        <Image width={16} height={16} style={{ marginLeft: "10px" }} quality={20} priority alt="Slack Logo" src={SlackLogo}></Image>
      </a>
    </header>
  );
}
