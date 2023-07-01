import Image from "next/image";
import Link from "next/link";
import styles from "./header.module.scss";
import React, { useState, useEffect, useRef } from "react";
import cx from "classnames";
const TrunkLogo = "/TrunkLogo.png";
const SlackLogo = "/slack_logo.png";
import Search from "../Search";
import { Extension } from "@/types";

interface HeaderProps {
  white?: boolean;
  search?: boolean;
  extensions?: Extension[];
}

export default function Header({ white = false, search = false, extensions = [] }: HeaderProps) {
  const [scrolled, setScrolled] = useState(false);
  const [showNavLinks, setShowNavLinks] = useState(false);

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
      <div className={styles.headerLeft}>
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
        <button onClick={() => setShowNavLinks(!showNavLinks)} className={styles.menuButton}>
          {showNavLinks ? "" : "Menu"}
        </button>
        {search && (
          <div className={styles.headerSearchCont}>
            <Search extensions={extensions}></Search>
          </div>
        )}
      </div>
      {/* <div> */}
      <div style={search ? { marginLeft: "auto", position: "relative" } : {}}>
        <div className={cx(styles.linksCont, showNavLinks ? styles.showLinks : "")}>
          <a
            className={styles.navLink}
            style={search ? { padding: "0 14px" } : {}}
            href={"https://github.com/tembo-io/trunk#installation"}
            target="_blank"
          >
            Download
          </a>
          <a
            style={search ? { padding: "0 14px" } : {}}
            className={styles.navLink}
            href="https://github.com/tembo-io/trunk"
            target="_blank"
          >
            Contribute
          </a>
          <a
            style={search ? { padding: "0 14px" } : {}}
            className={styles.navLink}
            href="https://tembo-io.github.io/trunk/"
            target="_blank"
          >
            Docs
          </a>
          {/* TODO: Add blog */}
          <Link className={styles.navLink} href={"/"}>
            Blog
          </Link>
        </div>
      </div>
      <a
        target="_blank"
        href="https://trunk-crew.slack.com/join/shared_invite/zt-1w6kv2plz-vJlgjXzTSALlvjJyU8lhcQ#/shared-invite/email"
        className={styles.ctaCont}
      >
        <span className={styles.ctaText}>Join Trunk on Slack</span>
        <Image width={16} height={16} style={{ marginLeft: "10px" }} quality={20} priority alt="Slack Logo" src={SlackLogo}></Image>
      </a>
      {/* </div> */}
    </header>
  );
}
