import type { InferGetStaticPropsType, GetStaticProps } from "next";
import Link from "next/link";
import styles from "./categories.module.scss";
import cx from "classnames";
import { Category } from "@/types";
import { useRouter } from "next/router";

export default function Categories({ categories }: { categories: Category[] }) {
  const router = useRouter();
  const { cat } = router.query;
  return (
    <div className={styles.container}>
      {categories
        .filter((item) => item.extension_count > 0)
        .map((item) => (
          <div key={item.slug} className={styles.listItem}>
            <Link shallow href={`/?cat=${item.slug}`} className={styles.interMed16}>
              {item.name}
            </Link>
            <span className={cx(styles.catCount, styles.interReg12, cat === item.slug ? styles.activeCat : "")}>
              {item.extension_count}
            </span>
          </div>
        ))}
    </div>
  );
}
