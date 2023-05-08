import { useState } from "react";
import Image from "next/image";
import { useRouter } from "next/router";
import styles from "./user.module.scss";
import { useUser, useAuth } from "@clerk/nextjs";
import { useMutation } from "@tanstack/react-query";
import Header from "@/components/Header";

export default function User() {
  const [apiToken, setApiToken] = useState("");
  const { isSignedIn, user } = useUser();
  const { getToken } = useAuth();

  const router = useRouter();
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

  if (!isSignedIn) {
    router.push("/");
  }

  return (
    <div>
      <Header></Header>
      <div className={styles.userCont}>
        <h1>Account</h1>

        <div className={styles.section}>
          {/* <h2 className={styles.sectionTitle}>Profile Information</h2> */}
          <div className={styles.profileRow}>
            {user?.profileImageUrl && (
              <Image style={{ borderRadius: "4px" }} src={user?.profileImageUrl} alt="user profile image" width={150} height={150}></Image>
            )}
            <div className={styles.nameBlock}>
              <h3 className={styles.fullName}>{user?.fullName}</h3>
              <h4 className={styles.userName}>{user?.username}</h4>
            </div>
          </div>
        </div>
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
      </div>
    </div>
  );
}
