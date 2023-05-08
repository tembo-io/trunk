import cx from "classnames";
import { useUser, useSignIn, useClerk } from "@clerk/nextjs";
import Image from "next/image";
import { Inter } from "next/font/google";
import Link from "next/link";
import styles from "./Header.module.scss";
const inter = Inter({ subsets: ["latin"], weight: ["400", "700"] });
import { useRouter } from "next/router";

export default function Header({ bgTransparent = false }) {
  const router = useRouter();
  const { signIn, isLoaded } = useSignIn();
  const { user } = useUser();
  const { signOut } = useClerk();

  const signInWithGitHub = () => {
    if (isLoaded) {
      signIn.authenticateWithRedirect({
        strategy: "oauth_github",
        redirectUrl: "/sso-callback",
        redirectUrlComplete: "/",
      });
    }
  };

  const handleSignout = () => {
    if (isLoaded) {
      signOut();
    }
    router.push("/");
  };

  return (
    <header className={styles.header} style={bgTransparent ? { background: "transparent" } : {}}>
      <Link href="/" style={{ textDecoration: "none" }}>
        <h1 className={cx(inter.className, styles.title)}>Trunk</h1>
      </Link>
      {user ? (
        <>
          <div className={styles.loggedIn}>
            <Link href={`/user/${user.username}`} className={cx(inter.className, styles.navLink)}>
              Profile
            </Link>
            <button onClick={handleSignout} className={cx(inter.className, styles.navLink)}>
              Logout
            </button>
          </div>
        </>
      ) : (
        <button onClick={() => signInWithGitHub()} className={styles.loginButton}>
          <Image src="/images/github.svg" alt="GitHub logo" width={20} height={20}></Image>
          <span className={cx(inter.className, styles.authText)}>Sign in with GitHub</span>
        </button>
      )}
    </header>
  );
}
