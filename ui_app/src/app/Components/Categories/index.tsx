"use client";
import Link from "next/link";
import styles from "./categories.module.scss";
import cx from "classnames";
import { useRouter } from "next/navigation";

async function getData() {
  const res = await fetch("https://registry.pgtrunk.io/categories/all");
  // The return value is *not* serialized
  // You can return Date, Map, Set, etc.

  // TODO: handle errors
  // Recommendation: handle errors
  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error("Failed to fetch data");
  }

  return res.json();
}

export default async function Categories() {
  const router = useRouter();
  const data = await getData();

  const sortedData = data.sort((a, b) => (a.name < b.name ? -1 : 1));

  const updateRouter = (slug) => {
    router.push(`?cat=${slug}`);
    console.log("SLUG", slug);
  };

  return (
    <div>
      <div className={styles.container}>
        {sortedData.map((item) => (
          <div key={item.slug} className={styles.listItem}>
            <p onClick={() => updateRouter(item.slug)} className={styles.interMed16}>
              {item.name}
            </p>
            <span className={cx(styles.catCount, styles.interReg12)}>{item.extension_count}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
