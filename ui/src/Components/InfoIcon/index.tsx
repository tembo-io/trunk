import React, { useState } from 'react';
import Image from 'next/image';
import styles from './info-icon.module.scss';

interface InfoIconProps {
  infoText: string;
}

const InformationIcon = '/information.png';

const InfoIcon: React.FC<InfoIconProps> = ({ infoText }) => {
  const [isHovered, setIsHovered] = useState(false);

  const showTip = () => {
    setIsHovered(true);
  };

  const hideTip = () => {
    setIsHovered(false);
  };

  return (
    <div
      className={styles.infoIcon}
      onMouseEnter={showTip}
      onMouseLeave={hideTip}>
      <div className={styles.infoIconContent}>
        <span>
          <Image
            className={styles.linkIcon}
            src={InformationIcon}
            width={14}
            height={14}
            alt="Information icon"
          />
        </span>
        {isHovered && (
          <div className={styles.Tooltip}>
            <div
              style={{
                whiteSpace: 'pre-wrap',
                overflowWrap: 'break-word',
                minWidth: '120px',
              }}>
              {infoText}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default InfoIcon;
