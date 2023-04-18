import languageDetector from '../lib/languageDetector'
import { useRouter } from 'next/router'
import Link from 'next/link'
import styles from '../styles/langpicker.module.css';

function FlagIcon({ countryCode = "" }) {
  if (countryCode === "en") {
    countryCode = "gb"
  }

  return (
    <span className={`fi fis ${styles.fiCircle} inline-block mr-2 fi-${countryCode}`} />
  )
}

const LanguageSwitchLink = ({ locale, ...rest }) => {
  const router = useRouter()

  let href = rest.href || router.asPath
  let pName = router.pathname
  Object.keys(router.query).forEach((k) => {
    if (k === 'locale') {
      pName = pName.replace(`[${k}]`, locale)
      return
    }
    pName = pName.replace(`[${k}]`, router.query[k])
  })
  if (locale) {
    href = rest.href ? `/${locale}${rest.href}` : pName
  }

  console.log("Locale: ", locale);
  console.log("pName: ", pName);

  return (
    <Link href={href} onClick={() => languageDetector.cache(locale)} className='flex justify-center items-center'>
      <FlagIcon countryCode={locale} />
      <span>{locale}</span>
    </Link>
  );
};

export default LanguageSwitchLink