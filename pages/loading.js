import Head from "next/head";
import styles from '../styles/animation.module.css';
import { useRouter } from "next/router";
import { useEffect, useContext } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { SettingsContext } from '../state/context';

export default function Loading() {
  const { settings } = useContext(SettingsContext);
  const activateLogging = settings["ActivateLogging"] != undefined ? settings["ActivateLogging"] : false;
  const obfuscatedMode = settings["ObfuscatedMode"] != undefined ? settings["ObfuscatedMode"] : true;
  const router = useRouter();
  let { query: { scan_path }, } = router;
  let db_location = "";

  useEffect(() => {
    setTimeout(scanning, 0);
  });

  const scanning = () => {
    if (activateLogging) {
      process.env.RUST_LOG = "info"; // Optionally Debug
    } else {
      process.env.RUST_LOG = "warn";
    }
    console.log("ENV is set on: ", process.env.RUST_LOG)
    if (typeof window !== "undefined") {
      invoke("start_scanner", {
        path: scan_path,
        dbfile: db_location,
        obfuscated: obfuscatedMode,
      })
        .then((message) => {
          console.log(message.length);
          console.log(typeof message);
          console.log(message);
          if (message && message.length > 0 && message != "[]") {
            console.log(message);
            router.push({
              pathname: '/infected',
              query: { virus_list: message }
            });
          } else {
            router.push("/clean");
          }
        })
        .catch((error) => {
          console.error(error);
          router.push({
            pathname: '/',
            query: { scanner_error: error }
          })
        });
    } else {
      console.error("Nextjs not in client mode!");
    }
  }

  return (
    <>
      <Head>
        <title>Loading...</title>
      </Head>
      <main className="flex flex-col items-center justify-center h-screen">
        <h1 className="text-2xl font-bold text-center">Loading... Please wait</h1>
        <div className="flex flex-row m-10">
          <div className={[styles.main_div, styles.zero_div].join(" ")}></div>
          <div className={[styles.main_div, styles.first_div].join(" ")}></div>
          <div className={[styles.main_div, styles.second_div].join(" ")}></div>
          <div className={[styles.main_div, styles.third_div].join(" ")}></div>
          <div className={[styles.main_div, styles.fourth_div].join(" ")}></div>
        </div>
      </main>
    </>
  )
}