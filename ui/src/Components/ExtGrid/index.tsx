"use client";
// import { useSearchParams } from "next/navigation";
import styles from "./extGrid.module.scss";
import { truncate } from "../../stringHelpers";
import cx from "classnames";

async function getData() {
  const res = await fetch("https://registry.pgtrunk.io/extensions/all");

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
export default function ExtGrid() {
  // const searchParams = useSearchParams();

  // const cat = searchParams.get("cat");

  // const data = await getData();
  const data = [];
  return (
    <div className={styles.container}>
      <div className={styles.sectionHeader}>
        <h1>Featured</h1>
        <div className={styles.inputCont}>
          <input type="text" className={styles.input} />
          <button className={cx(styles.searchButton, styles.interBold14)}>Search</button>
        </div>
      </div>
      <div className={styles.gridContainer}>
        {data.map((ext) => (
          <div key={ext.name} className={styles.extCard}>
            <div className={styles.titleRow}>
              <p>{ext.name}</p> <div className={styles.catBubble}>category</div>
            </div>
            <p className={styles.extDescription}>{truncate(ext.description)}</p>
            <p className={styles.extAuthor}>{ext.owners[0].userName}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
