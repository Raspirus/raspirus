import Head from "next/head";
import { useRouter } from "next/router";
import React, { useEffect, useContext, useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { SettingsContext } from '../state/context';
import { listen } from '@tauri-apps/api/event';
import { CircularProgressbar, buildStyles } from 'react-circular-progressbar';
import 'react-circular-progressbar/dist/styles.css';
import useTranslation from 'next-translate/useTranslation';


export default function Loading() {
  const { settings } = useContext(SettingsContext);
  const activateLogging = settings["ActivateLogging"] != undefined ? settings["ActivateLogging"] : false;
  const obfuscatedMode = settings["ObfuscatedMode"] != undefined ? settings["ObfuscatedMode"] : true;
  const [progress, setProgress] = useState(0);
  const router = useRouter();
  let { query: { scan_path }, } = router;
  let db_location = "";
  const t = useTranslation('common').t;

  useEffect(() => {
    const handleProgress = (event) => {
      console.log("Progress: ", event.payload.message);
      setProgress(event.payload.message);
    };
  
    const startListening = async () => {
      await listen('progress', handleProgress);
    };
  
    startListening();
  
    // Clean up function to remove the event listener when the component unmounts
    return () => {
      removeEventListener('progress', handleProgress);
    };
  }, []);

  useEffect(() => {
    scanning();
  }, []);

  const scanning = async () => {
    if (activateLogging) {
      process.env.RUST_LOG = "info";
    } else {
      process.env.RUST_LOG = "warn";
    }

    try {
      const message = await invoke("start_scanner", {
        path: scan_path,
        dbfile: db_location,
        obfuscated: obfuscatedMode,
      });

      if (message && message.length > 0 && message != "[]") {
        console.log(message);
        router.push({
          pathname: '/infected',
          query: { virus_list: message }
        });
      } else {
        router.push("/clean");
      }
    } catch (error) {
      console.error(error);
      localStorage.setItem("errorOccurred", 'true');
      router.push({
        pathname: '/',
        query: { scanner_error: error }
      })
    }
  };

  return (
    <>
      <Head>
        <title>{t('loading_title')}</title>
      </Head>
      <main className="flex flex-col items-center justify-center h-screen">
        <h1 className="text-2xl font-bold text-center">{t('loading_text')}</h1>
        <div className="flex flex-row m-10">
        <CircularProgressbar
          value={progress}
          text={`${progress}%`}
          styles={buildStyles({
            textColor: '#35c091',
            pathColor: '#ff3366',
            trailColor: '#d6d6d6'
          })}
        />
        </div>
      </main>
    </>
  )
}