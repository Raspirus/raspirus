import Head from "next/head";
import styles from '../styles/animation.module.css';
import { useRouter } from "next/router";
import { useEffect, useContext } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { SettingsContext } from '../state/context';

export default function Loading() {
  const { settings } = useContext(SettingsContext);
  const updateDatabase = settings.hasOwnProperty("UpdateDatabase") ? settings["UpdateDatabase"] : false;
  const activateLogging = settings.hasOwnProperty("ActivateLogging") ? settings["ActivateLogging"] : false;
  const obfuscatedMode = settings.hasOwnProperty("ObfuscatedMode") ? settings["Obfuscated;ode"] : false;
  const router = useRouter();
  let { query: { scan_path }, } = router;
  let progress = 0;
  let db_location = "";

  function scanning() {
    if (typeof window !== "undefined") {
      invoke("start_scanner", {
        path: scan_path,
        update: updateDatabase,
        dbfile: db_location,
      })
        .then((message) => {
          console.log("Message: ", message);

          router.push("/clean");
          /*
                          if (message != "None") {
                            router.push("/infected");
                          } else {
                            router.push("/clean");
                          }
                          */
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
    console.log("Finished scanning");
  }

  useEffect(() => {
    setTimeout(scanning, 0);
  }, []);

  return (
    <>
      <Head>
        <title>Loading...</title>
      </Head>
      <main className="flex flex-col items-center justify-center h-full">
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