import { useState, useRef, useEffect } from "react";
import cx from "classnames";
import { useUser, useSignIn, useClerk } from "@clerk/nextjs";
import Image from "next/image";
import { Inter } from "next/font/google";
import Link from "next/link";
import styles from "./Header.module.scss";
const inter = Inter({ subsets: ["latin"], weight: ["400", "700"] });
import { useRouter } from "next/router";

export default function Header({ bgTransparent = false }) {
  const [showNavLinks, setShowNavLinks] = useState(false);
  const router = useRouter();
  const { signIn, isLoaded } = useSignIn();
  const { user } = useUser();
  const { signOut } = useClerk();
  const navRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setShowNavLinks(false);
  }, [user]);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (navRef.current && !navRef.current.contains(event.target as Node)) {
        setShowNavLinks(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [navRef]);

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
    <header className={cx(styles.header, showNavLinks ? styles.open : "")} style={bgTransparent ? { background: "transparent" } : {}}>
      <Link className={styles.logo} href="/" style={{ textDecoration: "none", display: "flex", alignItems: "center" }}>
        <div className={styles.logoCont}>
          <Image src="/images/trunk_logo.svg" alt="Trunk logo" width={28} height={28}></Image>
        </div>
        <h1 className={cx(inter.className, styles.title)}>Trunk</h1>
        <span className={cx(inter.className, styles.version)}>BETA</span>
      </Link>
      <button onClick={() => setShowNavLinks(!showNavLinks)} className={styles.menuButton}>
        {showNavLinks ? "" : "Menu"}
      </button>
      <div ref={navRef} className={cx(styles.linksCont, showNavLinks ? styles.showLinks : "")}>
        <a className={cx(inter.className, styles.navLink)} href="https://coredb-io.github.io/trunk/">
          Documentation
        </a>
        {user && (
          <Link href={`/user/${user.username || user.externalAccounts[0].username}`} className={cx(inter.className, styles.navLink)}>
            Profile
          </Link>
        )}
        {user && (
          <button onClick={handleSignout} className={cx(inter.className, styles.navLink)}>
            Logout
          </button>
        )}
        {!user && (
          <button onClick={() => signInWithGitHub()} className={styles.loginButton}>
            <Image src="/images/github.svg" alt="GitHub logo" width={20} height={20}></Image>
            <span className={cx(inter.className, styles.authText)}>Sign in with GitHub</span>
          </button>
        )}
        {showNavLinks && (
          <button
            onClick={() => setShowNavLinks(false)}
            className={cx(inter.className, styles.navLink)}
            style={{ fontSize: "1rem", marginTop: "40px" }}
          >
            Close
          </button>
        )}
      </div>
    </header>
  );
}
