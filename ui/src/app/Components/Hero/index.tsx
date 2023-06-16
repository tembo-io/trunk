import Image from "next/image";
import styles from "./hero.module.scss";
export default function Hero() {
  return (
    <div className={styles.container}>
      <p className={styles.tag}>The Postgres Community&apos;s</p>
      <div className={styles.titleCont}>
        <h1 className={styles.ext}>
          Extension <span className={styles.reg}>Registry</span>
        </h1>
      </div>
      <img className={styles.image} src={"/hero_background.svg"} alt={"background image"} />
    </div>
  );
}
