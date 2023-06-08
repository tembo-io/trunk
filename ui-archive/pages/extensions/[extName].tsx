import { useRouter } from "next/router";
import styles from "./extensionDetail.module.scss";
import { useQuery } from "@tanstack/react-query";
import fetchExtensions from "@/lib/fetchExtensions";
import Header from "@/components/Header";
import Link from "next/link";
import LoadingSpinner from "@/components/LoadingSpinner";
import { ExtensionListing } from "@/types";
import { formatDateString } from "@/lib/formatDate";

export default function ExtensionDetail() {
  const router = useRouter();
  const { extName } = router.query;
  const { isLoading, data } = useQuery<ExtensionListing[]>(["extList"], fetchExtensions);

  if (isLoading || !data) {
    return (
      <div>
        <Header></Header>
        <LoadingSpinner size="lg"></LoadingSpinner>
      </div>
    );
  }

  const ext = data.find((item) => item.name === extName);

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

          {ext?.updatedAt && (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>last updated</p>
              <p className={styles.infoDetail}>{formatDateString(ext.updatedAt)}</p>
            </div>
          )}
          {ext?.license && (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>license</p>
              <p className={styles.infoDetail}>{ext?.license}</p>
            </div>
          )}
          {ext?.fileSize && (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>file size</p>
              <p className={styles.infoDetail}>{ext.fileSize}</p>
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
          {ext?.owners && ext?.owners?.length > 0 && (
            <div className={styles.aboutSection}>
              <p className={styles.infoPara}>author</p>
              {ext?.owners?.map(({ userName, userId }) => (
                <Link key={userId} href={`/user/${userName}`} className={styles.infoDetail}>
                  {userName}
                </Link>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
