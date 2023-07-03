import { useState, useEffect } from "react";
import { LRUCache } from "lru-cache";
import type { InferGetStaticPropsType, GetStaticProps } from "next";
import { Extension } from "@/types";
import ReactMarkdown from "react-markdown";
import styles from "./extension.module.scss";
import cx from "classnames";
import remarkGfm from "remark-gfm";
// import "github-markdown-css";

import { truncate } from "@/stringHelpers";
import { formatDateString } from "@/formatDate";
import Image from "next/image";
const Octocat = "/OctocatIcon.png";
const LinkIcon = "/LinkIcon.png";
import Header from "@/Components/Header";
const CopyIcon = "/copy.png";
import { useRouter } from "next/router";
import { App, Octokit } from "octokit";

const cache = new LRUCache({
  max: 200,
  ttl: 60 * 60 * 100,
});

export default function Page({ extension, readme, repoDescription, allExtensions }: InferGetStaticPropsType<typeof getStaticProps>) {
  const [showFeedback, setShowFeedback] = useState(false);
  const router = useRouter();
  useEffect(() => {
    if (showFeedback) {
      const timer = setTimeout(() => {
        setShowFeedback(false);
      }, 2000);

      return () => clearTimeout(timer);
    }
  }, [showFeedback]);

  if (!extension && !router.isFallback) {
    console.log("EXT MISSING DATA");
    return (
      <div>
        <h1>Error</h1>
      </div>
    );
  }

  if (router.isFallback) {
    return (
      <div>
        <h1>Loading...</h1>
      </div>
    );
  }

  const latestVersion: Extension = extension[extension.length - 1];
  const installText = `trunk install ${latestVersion.name}` ?? "";

  const handleCopy = async () => {
    try {
      navigator.clipboard.writeText(installText);
      setShowFeedback(true);
    } catch (error) {
      console.log(error);
    }
  };
  return (
    <div className={styles.pageCont}>
      <Header search extensions={allExtensions} white />
      <div className={styles.extHeaderRow}>
        <div style={{ maxWidth: "100%" }}>
          <h1 className={styles.title}>{latestVersion.name ?? ""}</h1>
          <p className={styles.description}>{repoDescription ?? ""}</p>
        </div>
        <div className={styles.installCont}>
          <p className={styles.installText}>Install</p>
          <div className={styles.installRow}>
            <div className={cx(styles.buttonFeedback, showFeedback ? styles.showButtonFeedback : "")}>Copied to clipboard!</div>
            <p className={styles.installCode}>{installText}</p>
            <button onClick={handleCopy} className={styles.copyButton}>
              <Image src={CopyIcon} width={18} height={18} alt="Copy button" />
            </button>
          </div>
        </div>
      </div>
      <div className={styles.container}>
        {readme && (
          <div className={styles.markdownCont}>
            {/* <div className={cx("markdown-body", styles.markdown)}>hi</div> */}
            <ReactMarkdown className={cx("markdown-body", styles.markdown)} remarkPlugins={[remarkGfm]}>
              {readme}
            </ReactMarkdown>
          </div>
        )}
        <div className={styles.infoSection}>
          <h2 className={styles.details}>Details</h2>
          <h3 className={styles.about}>About</h3>
          <div className={styles.infoDetails}>
            {latestVersion.categories.length > 0 && (
              <div className={styles.infoRow}>
                <span className={styles.infoTitle}>Category</span>
                <span className={styles.infoValue}>{latestVersion.categories[0]}</span>
              </div>
            )}
            <div className={styles.infoRow}>
              <span className={styles.infoTitle}>Version</span>
              <span className={styles.infoValue}>{latestVersion.version}</span>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.infoTitle}>Last updated</span>
              <span className={styles.infoValue}>{formatDateString(latestVersion.updatedAt)}</span>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.infoTitle}>License</span>
              <span className={styles.infoValue}>{latestVersion.license}</span>
            </div>
            <div className={styles.infoRow}>
              <span className={styles.infoTitle}>Owner</span>
              <span className={styles.infoValue}>{latestVersion.owners[0].userName}</span>
            </div>
            <div className={styles.buttonLinkCont}>
              {latestVersion.homepage && (
                <a href={latestVersion.homepage} target="_blank" className={styles.buttonLink}>
                  <Image className={styles.linkIcon} src={LinkIcon} width={14} height={14} alt="Link icon" />
                  <span className={styles.linkText}>{truncate(latestVersion.homepage, 555).replace("https://", "")}</span>
                </a>
              )}
              {latestVersion.repository && (
                <a href={latestVersion.repository} target="_blank" className={styles.buttonLink}>
                  <Image className={styles.linkIcon} src={Octocat} width={14} height={14} alt="Link icon" />

                  <span className={styles.linkText}>{truncate(latestVersion.repository, 5535).replace("https://", "")}</span>
                </a>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export async function getStaticPaths() {
  try {
    const extRes = await fetch(`https://registry.pgtrunk.io/extensions/all`);
    const extensions = await extRes.json();

    const paths = extensions.map((ext: Extension) => ({
      params: { ext: ext.name },
    }));

    const trimmedList = paths.slice(0, 5);
    console.log("********** BUILT PATHS **********");
    // return { paths: [{ params: { ext:  } }], fallback: false };
    return { paths, fallback: true };
    // return { paths: [], fallback: true };
  } catch (error) {
    console.log("ERROR BUILDING PATHS", error);
    return { paths: [] };
  }
}

export async function getStaticProps({ params }: { params: { ext: string } }) {
  const GITHUB_TOKEN = process.env.GITHUB_TOKEN;

  const octokit = new Octokit({
    auth: process.env.TOKEN,
  });

  try {
    const cacheKey = "trunkData";
    let allExtensions = [];
    let readme = "";
    let extension = null;
    let repoRes = null;
    let repoDescription = "";
    let readmeJson = "";

    try {
      const allExtRes = await fetch(`https://registry.pgtrunk.io/extensions/all`);
      allExtensions = await allExtRes.json();
    } catch (error: any) {
      console.log("********** ERROR GETTING ALL EXTS: **********", error.message);
      allExtensions = [];
    }

    try {
      const extRes = await fetch(`https://registry.pgtrunk.io/extensions/detail/${params.ext}`);
      extension = await extRes.json();
      console.log("********** GOT EXT DETAIL **********");
    } catch (error) {
      console.log("********** ERROR FETCHING DETAIL FROM TRUNK **********", params.ext);
      extension = null;
    }
    const latestVersion: Extension = extension ? extension[extension.length - 1] : null;
    console.log("********** EXTENSION **********");
    if (extension && latestVersion?.repository) {
      console.log("********** GETTING REPO **********");
      const repo = latestVersion.repository;
      const noGh = repo.split("https://github.com/")[1];
      const split = noGh.split("/");
      const githubReadmeUrl =
        split.length === 2
          ? `https://api.github.com/repos/${split[0]}/${split[1]}/readme`
          : `https://api.github.com/repos/${split[0]}/${split[1]}/readme/${split[2]}`;

      const githubRepoUrl = `https://api.github.com/repos/${split[0]}/${split[1]}`;

      try {
        repoRes = await fetch(githubRepoUrl, {
          headers: {
            Authorization: `token ${GITHUB_TOKEN}`,
          },
        });
        const repoJson = repoRes ? await repoRes.json() : null;
        repoDescription = repoJson?.description ?? "";
        console.log("********** GOT REPO **********");
      } catch (error: any) {
        console.log("********** ERROR FETCHING REPO **********", error.message, repo);
        repoRes = null;
      }

      try {
        const readmeRes = await fetch(githubReadmeUrl, {
          headers: {
            Authorization: `token ${GITHUB_TOKEN}`,
          },
        });
        readmeJson = await readmeRes.json();
      } catch (error: any) {
        readme = "";
        console.log("********** README FETCH ERROR **********", error.message, githubReadmeUrl);
      }

      try {
        readme = readmeJson ? Buffer.from(readmeJson.content, "base64").toString("utf-8") : "";
      } catch (error: any) {
        console.log("********** README PARSE ERROR **********", error.message, githubReadmeUrl);
      }
    }
    return { props: { extension, readme, repoDescription, allExtensions } };
  } catch (error: any) {
    console.log("********** STATIC PROPS ERROR **********", error.message, params);
    return { props: { extension: null, readme: "", repoDescription: "" } };
  }
}
