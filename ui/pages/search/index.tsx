import cx from "classnames";
import { Inter } from "next/font/google";
import Image from "next/image";
import { useRouter } from "next/router";
import styles from "./Footer.module.scss";

import Logo from "/public/images/Logo.svg";
import CoreDB from "/public/images/CoreDB.svg";
const inter = Inter({ subsets: ["latin"], weight: ["400"] });

export default function Footer() {
  const router = useRouter();
  console.log("PARAMS", router.query);

  return <div>SEARCH PAGE</div>;
}
