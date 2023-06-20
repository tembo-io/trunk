import Image from "next/image";
import styles from "./hero.module.scss";
const BgImage = "/hero_bg.png";
export default function Hero() {
  return (
    <div className={styles.container}>
      <p className={styles.tag}>The Postgres Community&apos;s</p>
      <div className={styles.titleCont}>
        <h1 className={styles.ext}>
          Extension <span className={styles.reg}>Registry</span>
        </h1>
      </div>
      <Image
        style={{ width: "100%", height: "100%" }}
        width={1000}
        height={1000}
        quality={100}
        priority={true}
        className={styles.image}
        // placeholder="blur"
        src={BgImage}
        alt={"background image"}
      />
    </div>
  );
}
