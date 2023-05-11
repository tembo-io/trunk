import { useState } from "react";
import { useRouter } from "next/router";
import styles from "./search.module.scss";
import Link from "next/link";
import cx from "classnames";
import { truncateString } from "@/lib/truncateString";

import { Inter } from "next/font/google";
import { useUser, useAuth } from "@clerk/nextjs";
import { useMutation, useQuery } from "@tanstack/react-query";
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
  console.log("query", query);

  const handleChange = (input: string) => {
    setSearchQuery(input);
    router.push(`/search?q=${input}`);
  };

  const filteredItems =
    data?.filter(
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
      {isLoading && <LoadingSpinner size="lg"></LoadingSpinner>}
      <div>
        <div className={styles.extensionList}>
          {filteredItems.map((ext) => {
            let extDate = "";
            if (ext?.updatedAt) {
              extDate = ext?.updatedAt.split(" +")[0];
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
                  <p className={styles.extLastUpdated}>{extDate ? `${formatDistanceToNow(new Date(extDate))} ago` : ""}</p>
                </div>
              </Link>
            );
          })}
        </div>
      </div>
    </div>
  );
}
