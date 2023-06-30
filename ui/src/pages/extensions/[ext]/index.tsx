import { useState, useEffect } from "react";
import type { InferGetStaticPropsType, GetStaticProps } from "next";
import { Extension } from "@/types";
import ReactMarkdown from "react-markdown";
import styles from "./extension.module.scss";
import cx from "classnames";
import remarkGfm from "remark-gfm";
import "github-markdown-css";
import { truncate } from "@/stringHelpers";
import { formatDateString } from "@/formatDate";
import Image from "next/image";
const Octocat = "/OctocatIcon.png";
const LinkIcon = "/LinkIcon.png";
import Header from "@/Components/Header";
const CopyIcon = "/copy.png";

export default function Page({ extension, readme, allExtensions, repoDescription, error }: InferGetStaticPropsType<typeof getStaticProps>) {
  const latestVersion: Extension = extension[extension.length - 1];
  const [showFeedback, setShowFeedback] = useState(false);

  useEffect(() => {
    if (showFeedback) {
      const timer = setTimeout(() => {
        setShowFeedback(false);
      }, 2000);

      return () => clearTimeout(timer);
    }
  }, [showFeedback]);

  if (error) {
    console.log("ERROR: ", error);
    return <div>404</div>;
  }

  const installText = `trunk install ${latestVersion.name}`;

  const handleCopy = async () => {
    try {
      navigator.clipboard.writeText(installText);
      setShowFeedback(true);
    } catch (error) {
      console.log(error);
    }
  };
  return (
    <>
      <Header search extensions={allExtensions} white />
      <div className={styles.extHeaderRow}>
        <div>
          <h1 className={styles.title}>{latestVersion.name}</h1>
          <p className={styles.description}>{repoDescription}</p>
        </div>
        <div>
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
        <div className={styles.markdownCont}>
          <ReactMarkdown className={cx("markdown-body", styles.markdown)} remarkPlugins={[remarkGfm]}>
            {readme}
          </ReactMarkdown>
        </div>
        <div className={styles.infoSection}>
          <h2 className={styles.details}>Details</h2>
          <h3 className={styles.about}>About</h3>
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
          {latestVersion.homepage && (
            <a href={latestVersion.homepage} target="_blank" className={styles.buttonLink}>
              <Image className={styles.linkIcon} src={LinkIcon} width={14} height={14} alt="Link icon" />
              {truncate(latestVersion.homepage, 35).replace("https://", "")}
            </a>
          )}
          {latestVersion.repository && (
            <a href={latestVersion.repository} target="_blank" className={styles.buttonLink}>
              <Image className={styles.linkIcon} src={Octocat} width={14} height={14} alt="Link icon" />

              {truncate(latestVersion.repository, 35).replace("https://", "")}
            </a>
          )}
        </div>
      </div>
    </>
  );
}

export async function getStaticPaths() {
  const extRes = await fetch(`https://registry.pgtrunk.io/extensions/all`);
  const extensions = await extRes.json();

  const paths = extensions.map((ext: Extension) => ({
    params: { ext: ext.name },
  }));

  return { paths, fallback: false };
}

export async function getStaticProps({ params }: { params: { ext: string } }) {
  try {
    const extRes = await fetch(`https://registry.pgtrunk.io/extensions/detail/${params.ext}`);
    const allExtRes = await fetch(`https://registry.pgtrunk.io/extensions/all`);
    const allExtensions = await allExtRes.json();
    const extension = await extRes.json();

    let readme = "";
    const latestVersion = extension[extension.length - 1];
    const repo = latestVersion.repository;
    const noGh = repo.split("https://github.com/")[1];
    const split = noGh.split("/");
    const apiUrl =
      split.length === 2
        ? `https://api.github.com/repos/${split[0]}/${split[1]}/readme`
        : `https://api.github.com/repos/${split[0]}/${split[1]}/readme/${split[2]}`;

    const repoUrl = `https://api.github.com/repos/${split[0]}/${split[1]}`;

    const repoRes = await fetch(repoUrl);
    const repoJson = await repoRes.json();
    const repoDescription = repoJson.description ?? "";
    const readmeRes = await fetch(apiUrl);
    const readmeJson = await readmeRes.json();
    readme = Buffer.from(readmeJson.content, "base64").toString("utf-8");

    return { props: { extension, readme, allExtensions, repoDescription } };
  } catch (error: any) {
    console.log("ERROR FETCHING", error.message);
    return { props: { extension: [], readme: "", allExtensions: [], repoDescription: "", error: error.message } };
  }
}
