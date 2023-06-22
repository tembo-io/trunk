import Image from "next/image";
import styles from "./hero.module.scss";
const BgImage = "/bg_text.png";
const TopSwoop = "/top_swoop.png";
const BottomSwoop = "/bottom_swoop.png";
export default function Hero() {
  return (
    <div className={styles.container}>
      <div className={styles.installCont}>
        <p className={styles.getTrunk}>Get Trunk</p>
        <div className={styles.installRow}>
          <div className={styles.installDropdown}>Cargo</div>
          <p className={styles.installCode}>cargo install pg-trunk</p>
          <button className={styles.copyButton}>Copy</button>
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
      />
    </div>
  );
}
