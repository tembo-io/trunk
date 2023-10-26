import styles from './loading.module.scss';
import React from 'react';
import Image from 'next/image';

export default function LoadingScreen() {
  return (
    <div className={styles.loadingScreen}>
      <img src="/anim.gif" alt="Loading..." />
    </div>
  );
}
