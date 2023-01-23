import '../styles/globals.css';
import Head from 'next/head';
import { Alert } from '../components/alert';
import "@fortawesome/fontawesome-svg-core/styles.css";
import { config } from "@fortawesome/fontawesome-svg-core";
config.autoAddCss = false;

export default function App({ Component, pageProps }) {
  return (
    <>
    <Head>
      <meta name="description" content="Raspirus TAURI frontend" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <link rel="icon" href="/favicon.ico" />
    </Head>
    <Alert />
    <Component {...pageProps} />
  </>
  )
}
