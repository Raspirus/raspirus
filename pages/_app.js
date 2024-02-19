import Head from 'next/head';
import "@fortawesome/fontawesome-svg-core/styles.css";
import "/node_modules/flag-icons/css/flag-icons.min.css";
import 'bootstrap/dist/css/bootstrap.min.css';
import '../styles/globals.css';
import { config } from "@fortawesome/fontawesome-svg-core";
import { Suspense } from 'react';
import { appWithTranslation } from 'next-i18next'
// Needs to be the first line to load included CSS
config.autoAddCss = false;

/**
 * This is a special file defined by Next.js that wraps each component for context or state.
 * It also provides a Head with metadata and a fallback loading screen if loading takes too much.
 * @param {*} param0 
 * @returns 
 */
const App = ({ Component, pageProps }) => {
  return (
    <>
      <Head>
        <meta name="description" content="Raspirus TAURI frontend" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Suspense fallback={<div>Loading...</div>}>
          <Component {...pageProps} />
      </Suspense>
    </>
  )
}

export default appWithTranslation(App)
