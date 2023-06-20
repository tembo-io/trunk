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
// type Extension = {

// }

export const getStaticProps: GetStaticProps<{
  categories: Category[];
}> = async () => {
  const catRes = await fetch("https://registry.pgtrunk.io/categories/all");
  const extRes = await fetch("https://registry.pgtrunk.io/extensions/all");

  const cats = await catRes.json();
  const exts = await extRes.json();

  const sortedData = cats.sort((a, b) => (a.name < b.name ? -1 : 1));

  const categoriesForGrid = {};
  cats.forEach((cat) => {
    categoriesForGrid[cat.slug] = { displayName: cat.name };
  });

  return { props: { categories: sortedData, extensions: exts, categoriesForGrid } };
};

export default function Home({
  categories,
  extensions,
  categoriesForGrid,
}: {
  categories: Category[];
  extensions: any[];
  categoriesForGrid: {};
}) {
  return (
    <div className={styles.main}>
      <Header />
      <Hero />
      <div className={styles.body}>
        <Categories categories={categories} />
        <ExtGrid extensions={extensions} categories={categories} categoriesForGrid={categoriesForGrid} />
      </div>
    </div>
  );
}
