import { useState, useEffect, useRef, RefObject } from 'react';
import { useRouter } from 'next/router';
import Link from 'next/link';
import cx from 'classnames';
import styles from './search.module.scss';
import { Extension } from '@/types';
import { truncate } from '@/stringHelpers';
import useExtList from '@/hooks/useExtList';

interface SearchProps {
  doOnClick?: () => void;
}

const Search: React.FC<SearchProps> = ({ doOnClick }) => {
  const [searchString, setSearchString] = useState('');
  const [selectedItemIndex, setSelectedItemIndex] = useState(-1);
  const [showResults, setShowResults] = useState(false);
  const router = useRouter();
  const {
    extensions,
    isLoading,
    isError,
  }: { extensions: Extension[]; isLoading: boolean; isError: boolean } =
    useExtList();

  const filteredList =
    isLoading || isError
      ? []
      : extensions.filter(
          (ext) =>
            ext.name.toLowerCase().includes(searchString.toLowerCase()) ||
            ext.description.toLowerCase().includes(searchString.toLowerCase())
        );

  const resultContainerRef: RefObject<HTMLInputElement> = useRef(null);

  useEffect(() => {
    document.addEventListener('click', handleClick);
    return () => {
      document.removeEventListener('click', handleClick);
    };
  }, []);

  const handleClick = (event: { target: any }) => {
    if (
      resultContainerRef.current &&
      !resultContainerRef.current.contains(event.target)
    ) {
      setShowResults(false);
    }
  };

  const handleKeyDown = (event: { key: string }) => {
    if (event.key === 'ArrowUp' && selectedItemIndex > 0) {
      setSelectedItemIndex(selectedItemIndex - 1);
    } else if (
      event.key === 'ArrowDown' &&
      selectedItemIndex < filteredList.length - 1
    ) {
      setSelectedItemIndex(selectedItemIndex + 1);
    } else if (event.key === 'Escape') {
      setShowResults(false);
    } else if (event.key === 'Enter') {
      if (selectedItemIndex > -1) {
        setShowResults(false);
        router.push(`/extensions/${filteredList[selectedItemIndex].name}`);
      } else {
        // router.push(`/search?q=${query}`);
      }
    }
  };

  return (
    <div className={styles.inputCont} ref={resultContainerRef}>
      <input
        type="text"
        className={cx(styles.input, styles.interReg16)}
        placeholder={`Search ${
          isLoading || isError ? '' : extensions.length
        } extensions`}
        value={searchString}
        onKeyDown={handleKeyDown}
        onChange={(e) => {
          setSearchString(e.target.value);
          setSelectedItemIndex(-1);
        }}
        onFocus={() => setShowResults(true)}
        onClick={() => {
          if (doOnClick) {
            doOnClick();
          }
        }}
      />
      <button className={cx(styles.interBold14, styles.searchButton)}>
        Search
      </button>
      {showResults && searchString.length > 0 && (
        <div className={styles.searchCont}>
          <ul className={styles.resultList}>
            {filteredList.map((ext, index) => (
              <li
                key={index}
                className={styles.resultItem}
                style={{
                  backgroundColor:
                    index === selectedItemIndex ? '#faac7f' : 'white',
                }}>
                <Link
                  className={styles.extLink}
                  href={`/extensions/${ext.name}`}>
                  <div>
                    <p className={styles.extName}>{ext.name}</p>
                    <p className={styles.extDesc}>
                      {truncate(ext.description)}
                    </p>
                  </div>
                  {ext?.categories[0] && (
                    <div className={styles.extCategory}>
                      {ext?.categories[0] ?? ''}
                    </div>
                  )}
                </Link>
                <hr className={styles.line} />
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
};

export default Search;
