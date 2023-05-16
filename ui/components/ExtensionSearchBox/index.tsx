import { useState, useEffect, useRef, RefObject } from "react";
import cx from "classnames";
import { Inter } from "next/font/google";
import { useRouter } from "next/router";
import Link from "next/link";
import styles from "./ExtensionSearchBox.module.scss";
import { useQuery } from "@tanstack/react-query";
import fetchExtensions from "@/lib/fetchExtensions";
import { ExtensionListing } from "@/types";
import { truncateString } from "@/lib/truncateString";
const inter = Inter({ subsets: ["latin"], weight: ["400", "700"] });

export default function ExtensionSearchBox() {
  const [query, setQuery] = useState("");
  const [selectedItemIndex, setSelectedItemIndex] = useState(-1);
  const [showresults, setShowResults] = useState(false);
  const router = useRouter();
  const { isLoading, data } = useQuery<ExtensionListing[]>(["extList"], fetchExtensions);
  const resultContainerRef: RefObject<HTMLInputElement> = useRef(null);

  useEffect(() => {
    document.addEventListener("click", handleClick);
    return () => {
      document.removeEventListener("click", handleClick);
    };
  }, []);

  const handleClick = (event: { target: any }) => {
    if (resultContainerRef.current && !resultContainerRef.current.contains(event.target)) {
      setShowResults(false);
    }
  };

  const filteredItems =
    data?.filter(
      (item) => item?.name.toLowerCase().includes(query.toLowerCase()) || item?.description?.toLowerCase().includes(query.toLowerCase())
    ) ?? [];

  const handleKeyDown = (event: { key: string }) => {
    if (event.key === "ArrowUp" && selectedItemIndex > 0) {
      setSelectedItemIndex(selectedItemIndex - 1);
    } else if (event.key === "ArrowDown" && selectedItemIndex < filteredItems.length - 1) {
      setSelectedItemIndex(selectedItemIndex + 1);
    } else if (event.key === "Enter") {
      if (selectedItemIndex > -1) {
        router.push(`/extensions/${filteredItems[selectedItemIndex].name}`);
      } else {
        router.push(`/search?q=${query}`);
      }
    }
  };

  return (
    <div className={styles.searchBoxCont} ref={resultContainerRef}>
      <div className={styles.searchCont}>
        <div className={styles.searchRow}>
          <input
            className={cx(inter.className, styles.input)}
            type="text"
            placeholder={isLoading ? "" : `Search ${data?.length} extensions`}
            value={query}
            onKeyDown={handleKeyDown}
            onFocus={() => setShowResults(true)}
            onChange={(e) => {
              setSelectedItemIndex(-1);
              setQuery(e.target.value);
            }}
          />
          <button onClick={() => router.push(`/search?q=${query}`)} className={cx(inter.className, styles.searchButton)}>
            Search
          </button>
        </div>
        <button className={cx(inter.className, styles.viewAll)} onClick={() => router.push("/search?q=")}>
          View all
        </button>
      </div>
      <div className={styles.resultContainer}>
        <ul className={styles.resultList}>
          {data &&
            showresults &&
            query.length > 0 &&
            filteredItems?.map((ext, index) => (
              <li
                style={{
                  backgroundColor: index === selectedItemIndex ? "#faac7f" : "white",
                }}
                className={styles.resultItem}
                key={ext.name}
              >
                <Link className={cx(inter.className, styles.extLink)} href={`/extensions/${ext.name}`}>
                  <div className={styles.extListHeader}>
                    {ext.name} <span className={styles.info}>v{ext.latestVersion}</span>
                  </div>
                  <div className={styles.info} style={{ marginLeft: 0 }}>
                    {truncateString(ext.description)}
                  </div>
                </Link>
              </li>
            ))}
        </ul>
      </div>
    </div>
  );
}
