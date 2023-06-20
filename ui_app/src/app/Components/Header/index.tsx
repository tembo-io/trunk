"use client";
import styles from "./header.module.scss";
import React, { useState, useEffect, useRef } from "react";
import cx from "classnames";

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
      <h1 className={styles.title}>header</h1>
    </header>
  );
}
