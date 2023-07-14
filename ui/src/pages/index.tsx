import { useState } from "react";
import type { InferGetStaticPropsType, GetStaticProps } from "next";
import Head from "next/head";
import styles from "./index.module.scss";
import Hero from "../Components/Hero";
import Categories from "../Components/Categories";
import ExtGrid from "../Components/ExtGrid";
import { Category, CategoriesForGrid, Extension } from "@/types";
import Header from "@/Components/Header";
export const getStaticProps: GetStaticProps<{
  categories: Category[];
}> = async () => {
  try {
    const catRes = await fetch("https://registry.pgtrunk.io/categories/all");
    const extRes = await fetch("https://registry.pgtrunk.io/extensions/all");

    const cats: Category[] = await catRes.json();
    const exts: Extension[] = await extRes.json();

    console.log("********** GOT EXT IN INDEX!!!!! **********", exts.length);

    const sortedData = cats.sort((a, b) => (a.name < b.name ? -1 : 1));

    const categoriesForGrid: CategoriesForGrid = {};
    cats.forEach((cat: Category) => {
      categoriesForGrid[cat.slug] = { displayName: cat.name };
    });

    return { props: { categories: sortedData, extensions: exts, categoriesForGrid } };
  } catch (error) {
    console.log("ERROR LOADING DATA: ", error);

    return { props: { categories: [], extensions: [], categoriesForGrid: {} } };
  }
};

export default function Home({
  categories,
  extensions,
  categoriesForGrid,
}: {
  categories: Category[];
  extensions: Extension[];
  categoriesForGrid: {};
}) {
  const [showMobileCategories, setShowMobileCategories] = useState(false);

  const showMobileCategoriesHandler = () => {
    window.scrollTo({ top: 0 });
    setShowMobileCategories(true);
  };
  return (
    <div>
      <Head>
        <title>Trunk</title>
        <meta name="description" content="Trunk is an open-source package installer and registry for PostgreSQL extensions." />
      </Head>
      <Header extensions={extensions}></Header>
      <div className={styles.main}>
        <Hero />
        <div className={styles.body}>
          <Categories categories={categories} showMobile={showMobileCategories} toggleCategoryMenu={setShowMobileCategories} />
          <ExtGrid
            extensions={extensions}
            categories={categories}
            categoriesForGrid={categoriesForGrid}
            setshowMobileCategories={showMobileCategoriesHandler}
          />
        </div>
      </div>
    </div>
  );
}
