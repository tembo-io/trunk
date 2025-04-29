import Image from 'next/image';
import Link from 'next/link';
import styles from './header.module.scss';
import React, { useState, useEffect, useRef } from 'react';
import cx from 'classnames';
const TrunkLogo = '/TrunkLogo.png';
const SlackLogo = '/slack_logo.png';
import Search from '../Search';
import { Extension } from '@/types';

interface HeaderProps {
  white?: boolean;
  search?: boolean;
  extensions?: Extension[];
}

export default function Header({ white = false, search = false }: HeaderProps) {
  const [scrolled, setScrolled] = useState(false);
  const [showNavLinks, setShowNavLinks] = useState(false);

  const headerRef = useRef(null);
  useEffect(() => {
    function handleScroll() {
      if (window.scrollY >= 20) {
        setScrolled(true);
      } else {
        setScrolled(false);
      }
    }
    window.addEventListener('scroll', handleScroll);
    return () => {
      window.removeEventListener('scroll', handleScroll);
    };
  }, []);
  return (
    <>
      <div className={styles.sunsetBanner}>
        We are sunsetting Trunk and ending active maintenance. The site and packages will stay accessible for some time.
      </div>
      <header
        style={white ? { backgroundColor: 'white' } : {}}
        className={cx(styles.header, scrolled ? styles.headerScrolled : '')}
        ref={headerRef}>
        <div className={styles.headerLeft}>
          <Link
            href={'/'}
            shallow
            style={{ display: 'flex', alignItems: 'center' }}>
            <Image
              width={102}
              height={38}
              style={{
                marginRight: '10px',
                minWidth: '102px',
                minHeight: '38px',
              }}
              quality={90}
              priority
              alt="Trunk Logo"
              src={TrunkLogo}></Image>
          </Link>
          <button
            onClick={() => setShowNavLinks(!showNavLinks)}
            className={styles.menuButton}>
            {showNavLinks ? (
              ''
            ) : (
              <span style={{ fontSize: '30px' }}>&#x2630;</span>
            )}
          </button>
          {search && (
            <div className={styles.headerSearchCont}>
              <Search />
            </div>
          )}
        </div>
        <div style={search ? { marginLeft: 'auto' } : {}}>
          <div
            className={cx(
              styles.linksCont,
              showNavLinks ? styles.showLinks : ''
            )}>
            <a
              className={cx(styles.navLink, search ? styles.navLinkSearch : '')}
              href={'https://github.com/tembo-io/trunk#installation'}
              target="_blank">
              Download
            </a>
            <a
              className={cx(styles.navLink, search ? styles.navLinkSearch : '')}
              href="https://github.com/tembo-io/trunk"
              target="_blank">
              Contribute
            </a>
            <a
              className={cx(styles.navLink, search ? styles.navLinkSearch : '')}
              href="https://tembo-io.github.io/trunk/"
              target="_blank">
              Docs
            </a>
            {/* TODO: Add blog */}
            <Link className={styles.navLink} href={'/'}>
              Blog
            </Link>
            <button
              onClick={() => setShowNavLinks(false)}
              className={styles.closeButton}>
              &#x2715;
            </button>
          </div>
        </div>
        <a
          target="_blank"
          href="https://join.slack.com/t/trunk-community/shared_invite/zt-1yiafma92-hFHq2xAN0ukjg_2AsOVvfg"
          className={styles.ctaCont}>
          <span className={styles.ctaText}>Join Trunk on Slack</span>
          <Image
            width={16}
            height={16}
            style={{ marginLeft: '10px' }}
            quality={20}
            priority
            alt="Slack Logo"
            src={SlackLogo}></Image>
        </a>
      </header>
    </>
  );
}
