import React from 'react';
import Dropdown from 'react-bootstrap/Dropdown';
import { useRouter } from 'next/router';
import LanguageSwitchLink from './LanguageSwitchLink'
import i18nextConfig from '../next-i18next.config'
import FlagIcon from './FlagIcon';

/**
 * This components serves as a Dropdown where users can select a language. Using the Rooter we set the language as a URL query
 * @returns a React Dropdown that allows users to select the language
 */
export default function LanguageSelector() {
  // We use the Router to determine from the URL what language was selected
  const router = useRouter()
  // Either use the language (locale) from the URL, or use the default one set in the config file (next-i18next.config.js)
  const currentLocale = router.query.locale || i18nextConfig.i18n.defaultLocale;
  // Loads all defined locales from the config file
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