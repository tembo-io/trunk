import {
  useRef,
  useEffect,
  RefObject,
  Dispatch,
  SetStateAction,
  MouseEventHandler,
} from 'react';

import styles from './extGrid.module.scss';
import { truncate } from '../../stringHelpers';
import cx from 'classnames';
import { useRouter } from 'next/router';
import Search from '../Search';
import { Category, Extension, CategoriesForGrid } from '@/types';
import Link from 'next/link';

function routerCatToString(input: string | string[] | undefined): string {
  if (typeof input === 'string') {
    return input;
  }
  return '';
}

export default function ExtGrid({
  extensions,
  categories,
  categoriesForGrid,
  setshowMobileCategories,
}: {
  extensions: Extension[];
  categories: Category[];
  categoriesForGrid: CategoriesForGrid;
  setshowMobileCategories?: MouseEventHandler<HTMLButtonElement>;
}) {
  const router = useRouter();
  const sectionTitleRef: RefObject<HTMLDivElement> = useRef(null);
  const title = router.query.cat
    ? categoriesForGrid[router.query.cat as string]?.displayName
    : 'All Extensions';

  const filteredList = router.query.cat
    ? extensions.filter((ext) => ext.categories.includes(title))
    : extensions;

  // Scroll to the start of the extensions panel
  function scrollToPanel() {
    if (sectionTitleRef.current) {
      window.scrollTo({
        top: sectionTitleRef.current.offsetTop - 35,
        behavior: 'smooth',
      });
    }
  }

  useEffect(() => {
    if (sectionTitleRef.current && router.query.cat) {
      scrollToPanel();
    }
  }, [router.query.cat]);

  return (
    <div className={styles.container} ref={sectionTitleRef}>
      <div className={styles.sectionHeader}>
        <h1 className={cx(styles.interMed24, styles.title)}>{title}</h1>
        <button
          className={styles.showCategoriesButton}
          onClick={setshowMobileCategories}>
          Categories
        </button>
        <Search doOnClick={scrollToPanel} />
      </div>
      <div className={styles.gridContainer}>
        {filteredList.map((ext) => (
          <button
            onClick={(event) => {
              const path = `/extensions/${ext.name}`;
              if (event.button === 1 || event.ctrlKey) {
                // Middle click or Alt+click
                window.open(`${window.location.origin}${path}`, '_blank');
              } else {
                router.push(path)
              }
            }}
            key={ext.name}
            className={styles.extCard}>
            <div className={styles.titleRow}>
              <p className={styles.interMed16}>{ext.name}</p>
            </div>
            {ext?.description && (
              <p className={cx(styles.interReg12, styles.description)}>
                {truncate(ext.description)}
              </p>
            )}
            {ext && (
              <CategoryTags
                extension={ext}
                categories={categories}
                selectedCategory={routerCatToString(router.query.cat)}
              />
            )}
          </button>
        ))}
      </div>
    </div>
  );
}

interface CategoryTagSearchProps {
  extension: Extension;
  categories: Category[];
  selectedCategory: string;
}

const CategoryTags: React.FC<CategoryTagSearchProps> = ({
  extension,
  categories,
  selectedCategory,
}) => {
  const getCategorySlug = (categoryName: string) => {
    return categories.find((cat) => cat.name === categoryName)?.slug;
  };

  // Take at most two categories of the extension
  function atMostTwoCategories() {
    return extension.categories.slice(0, 2);
  }

  return (
    <div className={styles.categoryTags}>
      {atMostTwoCategories().map((category, index) => (
        <Link
          key={index}
          shallow
          href={
            // Allow 'untoggling' a category: if we're already in the selected
            // category, go back home
            (() =>
              getCategorySlug(category) === selectedCategory
                ? '/'
                : `/?cat=${getCategorySlug(category)}`)()
          }
          onClick={(e) => e.stopPropagation()}
          className={styles.catBubble}>
          {category}
        </Link>
      ))}
    </div>
  );
};
