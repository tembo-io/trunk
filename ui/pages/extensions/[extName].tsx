import { useRouter } from "next/router";
import styles from "./extensionDetail.module.scss";
import { useQuery } from "@tanstack/react-query";
import fetchExtensions from "@/lib/fetchExtensions";
import Header from "@/components/Header";
import { formatDistanceToNow } from "date-fns";

import { ExtensionListing } from "@/types";

export default function ExtensionDetail() {
  const router = useRouter();
  const { extName } = router.query;
  const { isLoading, data } = useQuery<ExtensionListing[]>(["extList"], fetchExtensions);

  if (isLoading || !data) {
    return <div>Loading...</div>;
  }

  const ext = data.find((item) => item.name === extName);

  let extDate = "";
  if (ext?.updatedAt) {
    extDate = ext?.updatedAt.split(" +")[0];
  }

  return (
    <div>
      <Header></Header>
      <div className={styles.extDetailCont}>
        <div className={styles.titleSection}>
          <div className={styles.extHeaderRow}>
            <h1 className={styles.extTitle}>{ext?.name}</h1>
            <p className={styles.extVersion}>v{ext?.latestVersion}</p>
          </div>
          <p>{ext?.description}</p>
        </div>
        <div className={styles.section}>
          <h2 className={styles.sectionTitle}>Install</h2>
          <p className={styles.infoPara}>Run the following command in your project directory</p>
          <p className={styles.installCode}>trunk install {ext?.name}</p>
        </div>
        <div className={styles.section}>
          <h2 className={styles.sectionTitle}>About</h2>

          {ext?.updatedAt && extDate && (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>last updated</p>
              <p className={styles.infoDetail}>{formatDistanceToNow(new Date(ext?.updatedAt.split(" +")[0]))}</p>
            </div>
          )}

          {ext?.homepage && (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>homepage</p>
              <a href={ext.homepage} className={styles.infoDetail}>
                {ext.homepage}
              </a>
            </div>
          )}

          {ext?.repository && (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>repository</p>
              <a href={ext.repository} className={styles.infoDetail}>
                {ext.repository}
              </a>
            </div>
          )}
          {ext?.author ? (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>author</p>
              <p className={styles.infoDetail}>{ext.author}</p>
            </div>
          ) : (
            <div className={styles.aboutSection}>
              {/* <a href={"/"} className={styles.infoDetail}>
                Claim this extension
              </a> */}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
