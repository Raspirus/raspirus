import Head from "next/head";
import { useRouter } from "next/router";
import React, { useEffect, useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { CircularProgressbar, buildStyles } from 'react-circular-progressbar';
import 'react-circular-progressbar/dist/styles.css';
import { useTranslation } from 'next-i18next';
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic';
import Swal from "sweetalert2";

/**
 * Function that generates the necessary static paths and props manually
 * This is to fix an issue with next18 translations
 */
const getStaticProps = makeStaticProps('common');
export { getStaticPaths, getStaticProps }

/**
 * This page starts the scanner in the backend and then reads its incoming scanned percentage.
 * The percentage scanned is calculated on the backend and only needs to be displayed
 * @returns A full HTML page
 */
export default function Loading() {
  const [progress, setProgress] = useState(0);
  const router = useRouter();
  // Retrieves the query parameters
  let { query: { scan_path, hashcount }, } = router;
  // Can be empty, to let the backend choose the best one
  let db_location = "";
  const { t } = useTranslation('common');

  useEffect(() => {
    // Reads the emited progress signal from the backend
    const handleProgress = (event) => {
      //console.log("Progress: ", event.payload.message);
      setProgress(event.payload.message);
    };
    // Backend can also send error instead of the progress
    const handleProgressErr = (event) => {
      console.error(error);
      localStorage.setItem("errorOccurred", 'true');
      // Returns to the Home page with an error statements that will be displayed there
      router.push({
        pathname: '/',
        query: { scanner_error: event.payload.message }
      })
    }

    // Starts listening for incoming signals emited from the backend
    const startListening = async () => {
      await listen('progress', handleProgress);
      await listen('progerror', handleProgressErr);
    };

    startListening();

    // Clean up function to remove the event listener when the component unmounts
    return () => {
      removeEventListener('progress', handleProgress);
      removeEventListener('progerror', handleProgressErr);
    };
  }, [router]);

  useEffect(() => {
    scanning();
  }, []);

  // Starts the scan on the backend and periodically updates the frontend
  const scanning = async () => {
    if (parseInt(hashcount) <= 0) {
      // There are no signatures in the database, so the scanner cannot start
      isConfirmed = await Swal.fire({
        title: t('empty_db_dialog'),
        text: t('empty_db_dialog_text'),
        icon: "warning",
        showCancelButton: true,
        confirmButtonColor: "#3085d6",
        cancelButtonColor: "#d33",
        confirmButtonText: t('empty_db_dialog_confirm'),
        cancelButtonText: t('empty_db_dialog_cancel'),
      })

      if (isConfirmed) {
        router.push("/");
        return;
      }
    }
    try {
      // Start the function on the backend using Tauri
      const message = await invoke("start_scanner", {
        path: scan_path,
        dbfile: db_location,
      });
      // If the array of found viruses is not empty, redirected to the "infected" page
      if (message && message.length > 0 && message != "[]") {
        console.log(message);
        router.push({
          pathname: '/infected',
          query: { virus_list: message }
        });
      } else {
        // No virus found, device is clean
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