import type { InferGetStaticPropsType, GetStaticProps } from "next";
import Link from "next/link";
import styles from "./categories.module.scss";
import cx from "classnames";
import { Category } from "@/types";
import { useRouter } from "next/router";
import { Dispatch, SetStateAction } from "react";

export default function Categories({
  categories,
  showMobile,
  toggleCategoryMenu,
}: {
  categories: Category[];
  showMobile?: boolean;
  toggleCategoryMenu: Dispatch<SetStateAction<boolean>>;
}) {
  const router = useRouter();
  const { cat } = router.query;
  return (
    <div className={styles.container} style={showMobile ? { display: "flex" } : {}}>
      <div className={styles.mobileCategories}>
        <h1 className={styles.mobileTitle}>Categories</h1>
        <button onClick={() => toggleCategoryMenu(false)} className={styles.closeMenu}>
          &#x2715;
        </button>
      </div>
      {categories
        .filter((item) => item.extension_count > 0)
        .map((item) => (
          <Link onClick={() => toggleCategoryMenu(false)} shallow href={`/?cat=${item.slug}`} key={item.slug} className={styles.listItem}>
            <p className={styles.interMed16}>{item.name}</p>
            <span className={cx(styles.catCount, styles.interReg12, cat === item.slug ? styles.activeCat : "")}>
              {item.extension_count}
            </span>
          </Link>
        ))}
    </div>
  );
}
