import languageDetector from 'next-language-detector';
import i18nextConfig from '../next-i18next.config';

/*
*  This small helper function is part of the next18 workaround and provides the default languages
* set in the config file and the currently set one.
*/

export default languageDetector({
  supportedLngs: i18nextConfig.i18n.locales,
  fallbackLng: i18nextConfig.i18n.defaultLocale
})