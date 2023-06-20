import type { InferGetStaticPropsType, GetStaticProps } from "next";
import Image from "next/image";
import styles from "./index.module.scss";
import Header from "../Components/Header";
import Hero from "../Components/Hero";
import Categories from "../Components/Categories";
import ExtGrid from "../Components/ExtGrid";

type Category = {
  name: string;
  slug: string;
};

export const getStaticProps: GetStaticProps<{
  categories: Category[];
}> = async () => {
  console.log("GET STATIC PROPS");

  const res = await fetch("https://registry.pgtrunk.io/categories/all");
  console.log("RES", res);

  const cats = await res.json();
  const sortedData = cats.sort((a, b) => (a.name < b.name ? -1 : 1));
  return { props: { categories: sortedData } };
};

export default function Home({ categories }: { categories: Category[] }) {
  return (
    <main className={styles.main}>
      <Header />
      <Hero />
      <div className={styles.body}>
        <Categories categories={categories} />
        <ExtGrid />
      </div>
    </main>
  );
}
