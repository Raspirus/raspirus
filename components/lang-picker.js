import React from 'react';
import { Dropdown, DropdownButton } from 'react-bootstrap';
import i18nConfig from '../i18n.json';
import useTranslation from 'next-translate/useTranslation';
import styles from '../styles/langpicker.module.css';
import Link from 'next/link';

function FlagIcon({ countryCode = "" }) {
  if (countryCode === "en") {
    countryCode = "gb"
  }

  return (
    <span
      className={`fi fis ${styles.fiCircle} inline-block mr-2 fi-${countryCode}`}
    />
  )
}

export default function LanguageSelector() {
  const { locales, defaultLocale } = i18nConfig;
  const { t, lang } = useTranslation('common');

  return (
    <div>
      <DropdownButton id="dropdown-basic-button" title="Switch Lang">
        {locales.map(lng => {
          if (lng === lang) return null;
          return (
            <Dropdown.Item key={lng}>
              <Link href="/" locale={lng} key={lng}>
                <FlagIcon countryCode={lng} />
                {t(`common:language-name-${lng}`)}
              </Link>
            </Dropdown.Item>
          );
        })}
      </DropdownButton>
    </div>
  );
}
