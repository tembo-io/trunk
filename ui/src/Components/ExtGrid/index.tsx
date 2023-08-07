import { useRef, useEffect, RefObject, Dispatch, SetStateAction, MouseEventHandler } from "react";

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
  setshowMobileCategories,
}: {
  extensions: Extension[];
  categories: Category[];
  categoriesForGrid: CategoriesForGrid;
  setshowMobileCategories?: MouseEventHandler<HTMLButtonElement>;
}) {
  const router = useRouter();
  const sectionTitleRef: RefObject<HTMLDivElement> = useRef(null);
  const title = router.query.cat ? categoriesForGrid[router.query.cat as string]?.displayName : "All Extensions";

  const filteredList = router.query.cat ? extensions.filter((ext) => ext.categories.includes(title)) : extensions;

  // Scroll to the start of the extensions panel
  function scrollToPanel() {
    if (sectionTitleRef.current) {
      window.scrollTo({ top: sectionTitleRef.current.offsetTop - 35, behavior: "smooth" });
    }
  }

  useEffect(() => {
    if (sectionTitleRef.current && router.query.cat) {
      scrollToPanel();
    }
  }, [router.query.cat]);

  const getCategorySlug = (categoryName: string) => {
    return categories.find((cat) => cat.name === categoryName)?.slug;
  };

  return (
    <div className={styles.container} ref={sectionTitleRef}>
      <div className={styles.sectionHeader}>
        <h1 className={cx(styles.interMed24, styles.title)}>{title}</h1>
        <button className={styles.showCategoriesButton} onClick={setshowMobileCategories}>
          Categories
        </button>
        <Search doOnClick={scrollToPanel}/>
      </div>
      <div className={styles.gridContainer}>
        {filteredList.map((ext) => (
          <button onClick={() => router.push(`/extensions/${ext.name}`)} key={ext.name} className={styles.extCard}>
            <div className={styles.titleRow}>
              <p className={styles.interMed16}>{ext.name}</p>
            </div>
            <p className={cx(styles.interReg12, styles.description)}>{truncate(ext.description)}</p>
            {ext?.categories[0] && (
              <Link
                href={`/?cat=${getCategorySlug(ext.categories[0])}`}
                shallow
                onClick={(e) => e.stopPropagation()}
                className={styles.catBubble}
              >
                {ext.categories[0]}
              </Link>
            )}
          </button>
        ))}
      </div>
    </div>
  );
}
