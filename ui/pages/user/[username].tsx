import { useState } from "react";
import { useRouter } from "next/router";
import styles from "./user.module.scss";
import Link from "next/link";
import { useUser, useAuth } from "@clerk/nextjs";
import { useMutation, useQuery } from "@tanstack/react-query";
import Header from "@/components/Header";
import { ExtensionListing } from "@/types";
import fetchExtensions from "@/lib/fetchExtensions";
import { formatDistanceToNow } from "date-fns";
import LoadingSpinner from "@/components/LoadingSpinner";

export default function User() {
  const [apiToken, setApiToken] = useState("");
  const { isSignedIn, user } = useUser();
  const { getToken } = useAuth();
  const { isLoading, data } = useQuery<ExtensionListing[]>(["extList"], fetchExtensions);
  const router = useRouter();
  const { username } = router.query;

  const fetchApiToken = useMutation({
    mutationFn: async (body) => {
      const token = await getToken();
      return fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}/token/new`, {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
          "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
      });
    },
    onSuccess: async (data) => {
      const text = await data.text();
      setApiToken(text);
    },
    onError: (error) => {
      console.log("error", error);
      setApiToken("");
    },
  });

  const getApiToken = async () => {
    await fetchApiToken.mutateAsync();
  };

  const usersExtensions =
    data?.filter((ext) => {
      const isOwner = ext.owners.findIndex((owner) => owner.userName === username);
      return isOwner > -1;
    }) ?? [];

  const isSignedInUsersPage = isSignedIn && username === user.username;

  return (
    <div>
      <Header></Header>
      <div className={styles.userCont}>
        {isSignedInUsersPage ? <h1 className={styles.title}>Account</h1> : <h1 className={styles.title}>{username}</h1>}
        {isSignedInUsersPage && (
          <div className={styles.section}>
            <div className={styles.profileRow}>
              {user?.profileImageUrl && (
                // eslint-disable-next-line @next/next/no-img-element
                <img className={styles.profilePic} src={user.profileImageUrl} alt="User profile image" />
              )}
              <div className={styles.nameBlock}>
                <h3 className={styles.fullName}>{user?.fullName}</h3>
                <h4 className={styles.userName}>{user?.username}</h4>
              </div>
            </div>
          </div>
        )}
        {isLoading && <LoadingSpinner size="lg"></LoadingSpinner>}
        {usersExtensions.length > 0 && (
          <div className={styles.section}>
            <h2 className={styles.sectionTitle}>Owned Extensions</h2>
            {usersExtensions.map((ext) => {
              let extDate = "";
              if (ext?.updatedAt) {
                extDate = ext?.updatedAt.split(" +")[0];
              }
              return (
                <div key={ext.name} className={styles.ownedRow}>
                  <Link style={{ all: "unset", cursor: "pointer" }} href={`/extensions/${ext.name}`}>
                    <h3 className={styles.ownedTitle}>{ext.name}</h3>
                    <div className={styles.ownedMetaRow}>
                      <p className={styles.ownedDetail}>v{ext.latestVersion}</p>
                      <p className={styles.ownedDetail}>
                        {ext?.updatedAt ? `updated ${formatDistanceToNow(new Date(ext?.updatedAt.split(" +")[0]))} ago` : ""}
                      </p>
                    </div>
                  </Link>
                </div>
              );
            })}
          </div>
        )}
        {isSignedInUsersPage && (
          <div className={styles.section}>
            <h2 className={styles.sectionTitle}>API Token</h2>
            <p className={styles.infoPara}>
              You can use the API tokens generated on this page to run the commands that need write access to trunk. If you want to publish
              your own extensions then this is required.
            </p>
            {apiToken ? (
              <div>
                <p className={styles.infoPara}>
                  <strong>Your API token</strong>
                </p>
                <p className={styles.infoPara}>
                  To prevent keys being silently leaked they are stored on Trunk in hashed form. This means you can only download keys when
                  you first create them. If you have old unused keys you can safely delete them and create a new one.
                </p>
                <p className={styles.token}>{apiToken}</p>
              </div>
            ) : (
              <button onClick={getApiToken} className={styles.tokenButton}>
                {fetchApiToken.isLoading ? "Generating Token..." : "Create New Token"}
              </button>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
