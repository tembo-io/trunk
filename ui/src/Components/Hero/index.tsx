import { useState, useEffect } from "react";
import cx from "classnames";
import Image from "next/image";
import styles from "./hero.module.scss";
import BgImage from "../../../public/bg_text.png";

export default function Hero() {
  const [showFeedback, setShowFeedback] = useState(false);

  useEffect(() => {
    if (showFeedback) {
      const timer = setTimeout(() => {
        setShowFeedback(false);
      }, 2000);

      return () => clearTimeout(timer);
    }
  }, [showFeedback]);

  const handleCopy = async () => {
    try {
      navigator.clipboard.writeText("cargo install pg-trunk");
      setShowFeedback(true);
    } catch (error) {
      console.log(error);
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.installCont}>
        <p className={styles.getTrunk}>Get Trunk</p>
        <div className={styles.installRow}>
          <div className={cx(styles.buttonFeedback, showFeedback ? styles.showButtonFeedback : "")}>Copied to clipboard!</div>
          <div className={styles.installDropdown}>Cargo</div>
          <p className={styles.installCode}>cargo install pg-trunk</p>
          <button onClick={handleCopy} className={styles.copyButton}>
            Copy
          </button>
        </div>
        <p className={styles.installDesc}>Trunk is an open-source package installer and registry for PostgreSQL extensions.</p>
      </div>
      <Image
        style={{ width: "100%", height: "100%" }}
        width={1000}
        height={1000}
        quality={90}
        priority={true}
        className={styles.image}
        src={BgImage}
        alt={"background image"}
        placeholder="blur"
      />
    </div>
  );
}
