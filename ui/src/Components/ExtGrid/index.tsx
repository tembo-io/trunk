import styles from "./extGrid.module.scss";
import { truncate } from "../../stringHelpers";
import cx from "classnames";
import { useRouter } from "next/router";

export default function ExtGrid({ extensions, categories, categoriesForGrid }) {
  const router = useRouter();
  console.log(router.query);
  console.log(categoriesForGrid);

  const title = router.query.cat ? categoriesForGrid[router.query.cat].displayName : "Featured";

  const filteredList = extensions.filter((ext) => ext.categories.includes(title));

  return (
    <div className={styles.container}>
      <div className={styles.sectionHeader}>
        <h1>{title}</h1>
        <div className={styles.inputCont}>
          <input type="text" className={styles.input} />
          <button className={cx(styles.searchButton, styles.interBold14)}>Search</button>
        </div>
      </div>
      <div className={styles.gridContainer}>
        {filteredList.map((ext) => (
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
