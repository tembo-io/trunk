import { useState, useEffect, useRef } from "react";
import cx from "classnames";
import { useUser, useSignIn, useClerk } from "@clerk/nextjs";
import Image from "next/image";
import { Inter } from "next/font/google";
import { useRouter } from "next/router";
import Link from "next/link";
import styles from "./ExtensionSearchBox.module.scss";
const inter = Inter({ subsets: ["latin"], weight: ["400", "700"] });
import { useQuery, useMutation, useQueryClient, QueryClient, QueryClientProvider } from "@tanstack/react-query";
import fetchExtensions from "@/lib/fetchExtensions";

interface ExtensionListing {
  name: string;
  description: string;
  author: string;
  authorLink: string;
  isInstalled?: boolean;
  createdAt?: Date;
  latestVersion?: string;
  updatedAt?: Date;
  homepage?: string;
}
export default function ExtensionSearchBox() {
  const [query, setQuery] = useState("");
  const [selectedItemIndex, setSelectedItemIndex] = useState(-1);
  const [showresults, setShowResults] = useState(false);
  const router = useRouter();
  const { isLoading, data, isError, error } = useQuery<ExtensionListing[]>(["extList"], fetchExtensions);
  const resultContainerRef = useRef(null);

  useEffect(() => {
    document.addEventListener("click", handleClick);
    return () => {
      document.removeEventListener("click", handleClick);
    };
  }, []);

  const handleClick = (event) => {
    if (resultContainerRef.current && !resultContainerRef.current.contains(event.target)) {
      setShowResults(false);
    }
  };

  if (isLoading || !data) {
    return <div>Loading...</div>;
  }

  const filteredItems = data.filter((item) => item.name.toLowerCase().includes(query.toLowerCase()));

  const handleKeyDown = (event) => {
    if (event.key === "ArrowUp" && selectedItemIndex > 0) {
      setSelectedItemIndex(selectedItemIndex - 1);
    } else if (event.key === "ArrowDown" && selectedItemIndex < filteredItems.length - 1) {
      setSelectedItemIndex(selectedItemIndex + 1);
    } else if (event.key === "Enter") {
      if (selectedItemIndex > -1) {
        router.push(`/extensions/${filteredItems[selectedItemIndex].name}`);
      }
    }
  };

  const showResultsList = showresults && query.length > 0;
  console.log("showResultsList", showResultsList, showresults);

  return (
    <div className={styles.searchBoxCont} ref={resultContainerRef}>
      <div className={styles.searchRow}>
        <input
          type="text"
          value={query}
          onKeyDown={handleKeyDown}
          onFocus={() => setShowResults(true)}
          onChange={(e) => {
            setSelectedItemIndex(-1);
            setQuery(e.target.value);
          }}
        />
      </div>
      <div className={styles.resultContainer}>
        <ul className={styles.resultList}>
          {showresults &&
            query.length > 0 &&
            filteredItems?.map((ext, index) => (
              <li
                style={{
                  backgroundColor: index === selectedItemIndex ? "gray" : "white",
                }}
                key={ext.name}
              >
                <Link href={`/extensions/${ext.name}`}>{ext.name}</Link>
              </li>
            ))}
        </ul>
      </div>
    </div>
  );
}
