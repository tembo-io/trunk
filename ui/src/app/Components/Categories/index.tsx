import Link from "next/link";
import styles from "./categories.module.scss";

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
  const data = await getData();

  console.log(data);

  return (
    <ul className={styles.container}>
      {data.map((item) => (
        <li key={item.name}>
          <Link href={`?cat=${item.slug}`}>{item.name}</Link>
        </li>
      ))}
    </ul>
  );
}
