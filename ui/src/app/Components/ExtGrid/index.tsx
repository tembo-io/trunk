"use client";
import { useSearchParams } from "next/navigation";
import styles from "./extGrid.module.scss";
import { truncate } from "../../stringHelpers";

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
export default async function ExtGrid() {
  const searchParams = useSearchParams();

  const cat = searchParams.get("cat");

  const data = await getData();

  return (
    <div className={styles.container}>
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
  );
}
