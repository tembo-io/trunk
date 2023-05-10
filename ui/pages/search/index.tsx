import { useState } from "react";
import { useRouter } from "next/router";
import styles from "./search.module.scss";
import Link from "next/link";
import cx from "classnames";

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
      <div>
        <div>
          {/* {filteredItems.map((item) => (

          ))} */}
        </div>
      </div>
    </div>
  );
}
