import styles from "./footer.module.scss";
import Image from "next/image";
import TrunkLogo from "/public/TemboElephant.png";

export default function Footer() {
  return (
    <footer className={styles.footer}>
      <a className={styles.link} href="https://tembo.io" target="_blank">
        <span className={styles.small}> Sponsored by</span>
        <Image className={styles.image} src={TrunkLogo} width={20} height={20} alt="Tembo logo"></Image>Tembo
      </a>
    </footer>
  );
}
