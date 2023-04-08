import React from 'react';
import Dropdown from 'react-bootstrap/Dropdown';
import i18nConfig from '../i18n.json';
import useTranslation from 'next-translate/useTranslation';
import styles from '../styles/langpicker.module.css';
import Link from 'next/link';

function FlagIcon({ countryCode = "" }) {
  if (countryCode === "en") {
    countryCode = "gb"
  }

  return (
    <span className={`fi fis ${styles.fiCircle} inline-block mr-2 fi-${countryCode}`} />
  )
}

export default function LanguageSelector() {
  const { locales, defaultLocale } = i18nConfig;
  const { t, lang } = useTranslation('common');

  return (
    <div className='absolute top-0 left-0 m-2'>
      <Dropdown id="dropdown-basic-button" className='uppercase bg-white w-fit'>

        <Dropdown.Toggle id="dropdown-autoclose-true" className='text-black w-fit min-w-0'>
          <FlagIcon countryCode={lang} /> {lang.toUpperCase()}
        </Dropdown.Toggle>

        <Dropdown.Menu>
          {locales.map(lng => {
            return (
              <Dropdown.Item key={lng} as={'div'} className='h-10 w-fit'>
                <Link href="/" locale={lng} key={lng} className='flex justify-center items-center'>
                  <FlagIcon countryCode={lng} />
                  <span>{lng}</span>
                </Link>
                <br />
              </Dropdown.Item>
            );
          })}
        </Dropdown.Menu>
      </Dropdown>
    </div>
  );
}