import styles from '../styles/langpicker.module.css';

/**
 * This function uses the langpicker css style to determine the language to use from the countryCode given.
 * It displays a circle with the flag of said country
 * @param {String} countryCode The code of a country to determine its language in two characters format
 * @returns A span tag representing a circle with the flag of the given countrycode
 */
export default function FlagIcon({ countryCode = "" }) {
    // Determines if we want the us or gb flag for the english language
    if (countryCode === "en") {
      countryCode = "gb"
    }
  
    return (
      <span className={`fi fis ${styles.fiCircle} inline-block mr-2 fi-${countryCode}`} />
    )
  }