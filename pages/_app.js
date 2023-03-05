import '../styles/globals.css';
import Head from 'next/head';
import { useState } from 'react';
import "@fortawesome/fontawesome-svg-core/styles.css";
import { config } from "@fortawesome/fontawesome-svg-core";
import { SettingsContext } from '../state/context';
config.autoAddCss = false;

export default function App({ Component, pageProps }) {
  const [settings, setSettings] = useState({});

  return (
    <>
    <Head>
      <meta name="description" content="Raspirus TAURI frontend" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <link rel="icon" href="/favicon.ico" />
    </Head>
    <SettingsContext.Provider value={{ settings, setSettings }}>
      <Component {...pageProps} />
    </SettingsContext.Provider>
  </>
  )
}
