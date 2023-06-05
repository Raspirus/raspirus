import { useState, useEffect } from "react";

/*
* These simple helper functions gives access to the localstorage of the browser. This is mainly used
* to share error statements between pages and clear them afterwards. It is not used to store actual data, as
* that has been proven to not be reliable in an issue: https://github.com/Raspirus/Raspirus/issues/188
*/

/**
 * Internal function to retrieve data from localstorage by a given key
 * @param {String} key The key of the entry to identify it
 * @param {String} defaultValue A default value to use if no value or key was found
 * @returns the value found, or the default value
 */
function getStorageValue(key, defaultValue) {
  // getting stored value
  if (typeof window !== 'undefined') {
    const saved = localStorage.getItem(key);
    return saved || defaultValue;
  }
}

/**
 * Exported function to access the localstorage and retrieve data by a given key
 * @param {String} key The key to search the localstorage for
 * @param {String} defaultValue The default value to use if no value was found
 * @returns a function pair with the value and a function to set the value in the localstorage
 */
export const useLocalStorage = (key, defaultValue) => {
  const [value, setValue] = useState(() => {
    return getStorageValue(key, defaultValue);
  });

  useEffect(() => {
    // storing input name
    localStorage.setItem(key, value);
  }, [key, value]);

  return [value, setValue];
};