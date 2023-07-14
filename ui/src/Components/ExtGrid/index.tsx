import { useRef, useEffect, RefObject } from "react";

import styles from "./extGrid.module.scss";
import { truncate } from "../../stringHelpers";
import cx from "classnames";
import { useRouter } from "next/router";
import Search from "../Search";
import { Category, Extension, CategoriesForGrid } from "@/types";
import Link from "next/link";

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
  const sectionTitleRef: RefObject<HTMLDivElement> = useRef(null);
  const title = router.query.cat ? categoriesForGrid[router.query.cat as string]?.displayName : "All Extensions";

  const filteredList = router.query.cat ? extensions.filter((ext) => ext.categories.includes(title)) : extensions;

  useEffect(() => {
    if (sectionTitleRef.current && router.query.cat) {
      window.scrollTo({ top: sectionTitleRef.current.offsetTop - 35, behavior: "smooth" });
    }
  }, [router.query.cat]);

  const getCategorySlug = (categoryName: string) => {
    return categories.find((cat) => cat.name === categoryName)?.slug;
  };

  return (
    <div className={styles.container} ref={sectionTitleRef}>
      <div className={styles.sectionHeader}>
        <h1 className={cx(styles.interMed24, styles.title)}>{title}</h1>
        <Search />
      </div>
      <div className={styles.gridContainer}>
        {filteredList.map((ext) => (
          <button onClick={() => router.push(`/extensions/${ext.name}`)} key={ext.name} className={styles.extCard}>
            <div className={styles.titleRow}>
              <p className={styles.interMed16}>{ext.name}</p>
            </div>
            <p className={cx(styles.interReg12, styles.description)}>{truncate(ext.description)}</p>
            {ext?.categories[0] && (
              <Link href={`/?cat=${getCategorySlug(ext.categories[0])}`} onClick={(e) => e.stopPropagation()} className={styles.catBubble}>
                {ext.categories[0]}
              </Link>
            )}
          </button>
        ))}
      </div>
    </div>
  );
}
