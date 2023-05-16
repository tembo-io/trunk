import { useState } from "react";
import { useRouter } from "next/router";
import styles from "./search.module.scss";
import Link from "next/link";
import cx from "classnames";
import { truncateString } from "@/lib/truncateString";

import { Inter } from "next/font/google";
import { useQuery } from "@tanstack/react-query";
import Header from "@/components/Header";
import { ExtensionListing } from "@/types";
import fetchExtensions from "@/lib/fetchExtensions";
import { formatDistanceToNow } from "date-fns";
import LoadingSpinner from "@/components/LoadingSpinner";
const inter = Inter({ subsets: ["latin"], weight: ["400", "700"] });

export default function SearchPage() {
  const router = useRouter();
  const { q: query } = router.query;
  const [searchQuery, setSearchQuery] = useState<string>(query?.toString() ?? "");
  const { isLoading, data } = useQuery<ExtensionListing[]>(["extList"], fetchExtensions);

  const handleChange = (input: string) => {
    setSearchQuery(input);
    router.push(`/search?q=${input}`);
  };
  const sortedExtensions = data?.sort((a, b) => {
    if (a.name < b.name) {
      return -1;
    }
    return 1;
  });

  const filteredItems =
    sortedExtensions?.filter(
      (item) =>
        item?.name.toLowerCase().includes(searchQuery?.toLowerCase()) ||
        item?.description?.toLowerCase().includes(searchQuery?.toLowerCase())
    ) ?? [];

  return (
    <div>
      <Header></Header>
      <div className={styles.searchRow}>
        <input
          className={cx(inter.className, styles.input)}
          type="text"
          placeholder="Search extensions"
          value={searchQuery}
          onChange={(e) => handleChange(e.target.value.toString())}
        />
        <button className={cx(inter.className, styles.searchButton)}>Search</button>
      </div>
      {searchQuery && (
        <div style={{ textAlign: "center" }}>
          <button className={cx(inter.className, styles.viewAll)} onClick={() => setSearchQuery("")}>
            View all
          </button>
        </div>
      )}
      {isLoading && <LoadingSpinner size="lg"></LoadingSpinner>}
      <div>
        <div className={styles.extensionList}>
          {filteredItems.map((ext) => {
            let extDate = "";
            if (ext?.updatedAt) {
              extDate = ext?.updatedAt.split(".")[0];
            }
            return (
              <Link href={`/extensions/${ext.name}`} key={ext.name} className={styles.extCont}>
                <div className={styles.titleRow}>
                  <p className={styles.extName}>{ext.name}</p>
                  <p className={styles.extVersion}>v{ext.latestVersion}</p>
                </div>
                <p className={styles.extDesc}>{truncateString(ext.description, 200)}</p>
                <div className={styles.extFooter}>
                  <p className={styles.extAuthor}>
                    {ext.owners.map((owner) => (
                      <span key={owner.userId}>{owner.userName}</span>
                    ))}
                  </p>
                  <p className={styles.extLastUpdated}>
                    {extDate ? `${formatDistanceToNow(new Date(extDate.replace(/-/g, "/")))} ago` : ""}
                  </p>
                </div>
              </Link>
            );
          })}
          {filteredItems.length === 0 && !isLoading && (
            <div style={{ textAlign: "center" }}>
              <h1 className={cx(inter.className)}>We couldn&apos;t find any extensions for that query.</h1>
              <p className={cx(inter.className)}>Try searching again or creating your own</p>
              <div className={styles.linkRow}>
                <a href="https://coredb-io.github.io/trunk/" className={styles.linkButton}>
                  <span className={cx(inter.className, styles.repoText)}>Get Started</span>
                </a>
                <a href="https://github.com/CoreDB-io/trunk" className={styles.linkButton}>
                  <span className={cx(inter.className, styles.repoText)}>Contribute</span>
                </a>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
