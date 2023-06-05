import languageDetector from '../lib/languageDetector';
import { useRouter } from 'next/router';
import Link from 'next/link';
import FlagIcon from './FlagIcon';

/**
 * This function builds a link for the Router to change the language with it
 * @param {String} locale The selected locale (language)
 * @param {any} params Any other parameters that might be in the URL
 * @returns A cliccable Nextjs Link object that reloads the page with the selected language
 */
const LanguageSwitchLink = ({ locale, ...rest }) => {
  const router = useRouter()
  // Saves the original URL parameters
  let href = rest.href || router.asPath
  let pName = router.pathname
  Object.keys(router.query).forEach((k) => {
    // Searches for the 'locale' param
    if (k === 'locale') {
      pName = pName.replace(`[${k}]`, locale)
      return
    }
    pName = pName.replace(`[${k}]`, router.query[k])
  })
  // If a locale was found, build a new URL with it
  if (locale) {
    href = rest.href ? `/${locale}${rest.href}` : pName
  }

  return (
    <Link href={encodeURI(href)} onClick={() => languageDetector.cache(locale)} className='flex justify-center items-center'>
      <FlagIcon countryCode={locale} />
      <span>{locale}</span>
    </Link>
  );
};

export default LanguageSwitchLink