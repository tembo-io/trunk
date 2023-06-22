import { useState } from "react";
import styles from "./extGrid.module.scss";
import { truncate } from "../../stringHelpers";
import cx from "classnames";
import { useRouter } from "next/router";
import Search from "../Search";
import { Category, Extension, CategoriesForGrid } from "@/types";

export default function ExtGrid({
  extensions,
  categories,
  categoriesForGrid,
}: {
  extensions: Extension[];
  categories: Category[];
  categoriesForGrid: CategoriesForGrid;
}) {
  const router = useRouter();

  const title = router.query.cat ? categoriesForGrid[router.query.cat as string]?.displayName : "All Extensions";

  const filteredList = router.query.cat ? extensions.filter((ext) => ext.categories.includes(title)) : extensions;

  return (
    <div className={styles.container}>
      <div className={styles.sectionHeader}>
        <h1 className={styles.interMed24}>{title}</h1>
        <Search extensions={extensions}></Search>
      </div>
      <div className={styles.gridContainer}>
        {filteredList.map((ext) => (
          <div key={ext.name} className={styles.extCard}>
            <div className={styles.titleRow}>
              <p className={styles.interMed16}>{ext.name}</p>
              {ext?.categories[0] && <div className={styles.catBubble}>{ext.categories[0]}</div>}
            </div>
            <p className={cx(styles.interReg12, styles.description)}>{truncate(ext.description)}</p>
            <p className={cx(styles.interReg12, styles.author)}>{ext.owners[0].userName}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
