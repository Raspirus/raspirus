import Head from 'next/head';
import SettingComp from '../../components/SettingsCard';
import { useRouter } from 'next/router';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faFileLines, faUserNinja, faWrench, faHome, faClock } from '@fortawesome/free-solid-svg-icons';
import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom';
import Swal from 'sweetalert2';
import withReactContent from 'sweetalert2-react-content';
import moment from "moment";
import { useTranslation } from 'next-i18next';
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic';
import DateTimeSelector from '../../components/TimePicker';
import WeekdaySelector from '../../components/WeekdaySelector';
import schedule from 'node-schedule';
import { CircularProgressbar, buildStyles } from 'react-circular-progressbar';
import 'react-circular-progressbar/dist/styles.css';

/**
 * Function that generates the necessary static paths and props manually
 * This is to fix an issue with next18 translations
 */
const getStaticProps = makeStaticProps('common')
export { getStaticPaths, getStaticProps }

/**
 * Represents the settings page, a list of Material cards where the user can manage some simple settings
 * for the app. Each setting has its own card generated by the SettingComponent.
 * @returns A full HTML page
 */
export default function Settings() {
  const router = useRouter();
  const { t } = useTranslation('common');
  // Data for some of the settings is retrieved directly from the backend and saved back to it
  const [hash_count, setCount] = useState(0);
  const [updated_date, setDate] = useState(t('update_db_status_1'));
  const [auto_time, setAutotime] = useState('22:00');
  const [selectedWeekday, setSelectedWeekday] = useState(-1);
  const [cronjob, setcronjob] = useState(null);
  const [logging, setLogging] = useState(false);
  const [obfuscated, setObfuscated] = useState(false);
  // DB Update progress
  const [progress, setProgress] = useState(0);
  let db_location = "";

  // When the user goes back to the Home page, an update of the set settings
  // is sent to the backend, which then saves it in a local file
  const backHome = () => {
    if (updated_date == t('update_db_status_1')) {
      setDate("Never");
    }
    saveSettings();
    router.push('/');
  };

  /**
   * Function to save the data to the backend
   * It basically stringifys the data and sends it to the backend using Tauri.
   * The data is then saved in a local file on the device
   */
  const saveSettings = () => {
    const jsonData = {
      hashes_in_db: hash_count,
      last_db_update: updated_date,
      logging_is_active: logging,
      obfuscated_is_active: obfuscated,
      db_update_weekday: selectedWeekday,
      db_update_time: auto_time
    }
    const jsonString = JSON.stringify(jsonData);
    console.log("Client sends: ", jsonData);

    if (typeof window !== "undefined") {

      invoke("create_config", { contents: jsonString })
        .then((output) => {
          const parsedData = JSON.parse(output);
          console.log("Server answer: ", parsedData);
        })
        .catch((err) => console.error(err))
    }
  }

  useEffect(() => {
    // Reads the emited progress signal from the backend
    const handleProgress = (event) => {
      console.log("Progress: ", event.payload.message);
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
  }, [router])

  useEffect(() => {
    if (typeof window !== "undefined") {

      // Tries to create the config file on the backend, which returns the new created data
      // or the config found. This data then updates the frontend and is displayed
      invoke("create_config", {})
        .then((output) => {
          const parsedData = JSON.parse(output);
          setCount(parsedData.hashes_in_db);
          if (parsedData.last_db_update != "Never") {
            setDate(parsedData.last_db_update);
          }
          setLogging(parsedData.logging_is_active);
          setObfuscated(parsedData.obfuscated_is_active);
          setSelectedWeekday(parsedData.db_update_weekday);
          setAutotime(parsedData.db_update_time);
        })
        .catch((err) => console.error(err))
    }
  }, []);

  /**
   * Function to update the DB from the Settings page. 
   */
  const updating = () => {
    if (typeof window !== "undefined") {
      // Creates a pop-up with an indefinite loading animation
      Swal.fire({
        title: t('update_db_loading'),
        text: t('update_db_loading_val'),
        iconHtml: '<img src=../images/loading-anim.gif>',
        allowOutsideClick: false,
        showConfirmButton: false,
        allowEscapeKey: true,
        allowEnterKey: false
      })
      invoke("update_database", {
        dbfile: db_location,
      })
        .then((message) => {
          // If the update was successfull, update the data
          console.log(message);
          setCount(Number(message));
          setDate(moment().format("DD/MM/YYYY hh:mm:ss"));
          Swal.fire(t('update_db_completed'), t('update_db_completed_val'), "success");
        })
        .catch((error) => {
          console.error(error);
          // On error, set the failed update status as Date
          setDate(t('update_db_status_2'));
          Swal.fire(t('update_db_failed'), t('update_db_failed_val'), "error");
        });
    } else {
      console.error("Nextjs not in client mode!");
      Swal.fire(t('client_mode_error'), t('client_mode_error_val'), "error");
    }
  }

  /**
   * CURRENTLY NOT FULLY WORKING
   * Creates a Cronjob to update the database on a specific schedule.
   */
  const updateSchedule = () => {
    const [hours, minutes] = auto_time.split(':');
    const weekday = selectedWeekday;

    if (cronjob == null) {
      const job = schedule.scheduleJob('DataUpdater', { minute: minutes, hour: hours, dayOfWeek: weekday > 0 ? weekday : null }, () => { updating })
      setcronjob(job);
    } else {
      cronjob.cancel;
      schedule.gracefulShutdown()
        .then(_ => {
          const job = schedule.scheduleJob('DataUpdater', { minute: minutes, hour: hours, dayOfWeek: weekday > 0 ? weekday : null }, () => { updating })
          setcronjob(job);
        })
        .catch(err => console.error("Cronjob got canceled: ", err))
    }
  }

  return (
    <>
      <Head>
        <title>{t('settings_title')}</title>
      </Head>
      <div className="align-middle">
        <button
          onClick={backHome}
          type="button"
          className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md"
        >
          <FontAwesomeIcon
            icon={faHome}
            size="1x"
            className="pr-1"
          />
          {t('back_btn')}
        </button>
        <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
          {t('settings_title')}
        </h1>
      </div>

      <SettingComp
        title={t('update_db')}
        short={t('activate_logs_val')}
        short2={`${t('update_db_1')}: ${hash_count} | ${t('update_db_2')}: ${updated_date}`}
        icon={faWrench}
        action={updating}
        action_val={t('update_db_btn')}
        isOn={false}
      />

      <SettingComp
        title={t('activate_logs')}
        short={t('activate_logs_val')}
        icon={faFileLines}
        isOn={logging}
        setIsOn={setLogging}
      />
      <SettingComp
        title={t('obfuscated_mode')}
        short={t('obfuscated_mode_val')}
        icon={faUserNinja}
        isOn={obfuscated}
        setIsOn={setObfuscated}
      />

      <SettingComp
        title={t('auto_db')}
        short={t('auto_db_val')}
        short2={<><WeekdaySelector selectedWeekday={selectedWeekday} setSelectedWeekday={setSelectedWeekday} /><DateTimeSelector time={auto_time} setTime={setAutotime} /></>}
        icon={faClock}
        isOn={true}
        action={updateSchedule}
        action_val={t('auto_db_btn')}
      />
    </>
  );
}