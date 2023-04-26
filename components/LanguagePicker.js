import React from 'react';
import Dropdown from 'react-bootstrap/Dropdown';
import { useRouter } from 'next/router';
import LanguageSwitchLink from './LanguageSwitchLink'
import i18nextConfig from '../next-i18next.config'
import styles from '../styles/langpicker.module.css';

function FlagIcon({ countryCode = "" }) {
  if (countryCode === "en") {
    countryCode = "gb"
  }

  return (
    <span className={`fi fis ${styles.fiCircle} inline-block mr-2 fi-${countryCode}`} />
  )
}

export default function LanguageSelector() {
  const router = useRouter()
  const currentLocale = router.query.locale || i18nextConfig.i18n.defaultLocale;
  const locales = i18nextConfig.i18n.locales;


  return (
    <div className='absolute top-0 left-0 m-2'>
      <Dropdown id="dropdown-basic-button" className='uppercase bg-white w-fit'>

        <Dropdown.Toggle id="dropdown-autoclose-true" className='text-black w-fit min-w-0'>
          <FlagIcon countryCode={currentLocale} /> {currentLocale.toUpperCase()}
        </Dropdown.Toggle>

        <Dropdown.Menu>
          {locales.map(lng => {
            return (
              <Dropdown.Item key={lng} as={'div'} className='h-10 w-fit'>
                <LanguageSwitchLink 
                locale={lng}
                key={lng}
                />
                <br />
              </Dropdown.Item>
            );
          })}
        </Dropdown.Menu>
      </Dropdown>
    </div>
  );
}