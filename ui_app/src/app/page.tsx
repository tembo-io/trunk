"use client";
import Image from "next/image";
import styles from "./page.module.scss";
import Header from "./Components/Header";
import Hero from "./Components/Hero";
import Categories from "./Components/Categories";
import ExtGrid from "./Components/ExtGrid";

export default async function Home() {
  return (
    <main className={styles.main}>
      <Header />
      <Hero />
      <div className={styles.body}>
        <Categories />
        <ExtGrid />
      </div>
    </main>
  );
}
