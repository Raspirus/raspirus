import Head from 'next/head';
import SettingComp from '../../components/SettingsCard';
import { useRouter } from 'next/router';
import { invoke } from "@tauri-apps/api/tauri";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faFileLines, faUserNinja, faWrench, faHome, faClock } from '@fortawesome/free-solid-svg-icons';
import React, { useState } from 'react';
import Swal from 'sweetalert2';
import moment from "moment";
import { useTranslation } from 'next-i18next';
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic';
import DateTimeSelector from '../../components/TimePicker';
import WeekdaySelector from '../../components/WeekdaySelector';
import schedule from 'node-schedule';

const getStaticProps = makeStaticProps('common')
export { getStaticPaths, getStaticProps }

export default function Settings() {
  const router = useRouter();
  const { t } = useTranslation('common');
  const [hash_count, setCount] = useState(0);
  const [updated_date, setDate] = useState(t('update_db_status_1'));
  const [auto_time, setAutotime] = useState('22:00');
  const [selectedWeekday, setSelectedWeekday] = useState(-1);
  const [cronjob, setcronjob] = useState(null);
  let db_location = "";

  const backHome = () => {
    router.push('/');
  };

  const updating = () => {
    if (typeof window !== "undefined") {
      Swal.fire({
        title: t('update_db_loading'),
        text: t('update_db_loading_val'),
        iconHtml: '<img src=../images/loading-anim.gif>',
        allowOutsideClick: false,
        showConfirmButton: false,
        allowEscapeKey: false,
        allowEnterKey: false
      })
      invoke("update_database", {
        dbfile: db_location,
      })
        .then((message) => {
          console.log(message);
          setCount((JSON.parse(message)).toLocaleString('en'));
          setDate(moment().format("DD/MM/YYYY hh:mm:ss"));
          Swal.fire(t('update_db_completed'), t('update_db_completed_val'), "success");
        })
        .catch((error) => {
          console.error(error);
          setDate(t('update_db_status_2'));
          Swal.fire(t('update_db_failed'), t('update_db_failed_val'), "error");
        });
    } else {
      console.error("Nextjs not in client mode!");
      Swal.fire(t('client_mode_error'), t('client_mode_error_val'), "error");
    }
  }

  const updateSchedule = () => {
    const [hours, minutes] = auto_time.split(':');
    const weekday = selectedWeekday;

    if (cronjob == null) {
      const job = schedule.scheduleJob('DataUpdater', {minute: minutes, hour: hours, dayOfWeek: weekday > 0 ? weekday : null}, () => {updating})
      setcronjob(job);
    } else {
      cronjob.cancel;
      schedule.gracefulShutdown()
        .then(_ => {
          const job = schedule.scheduleJob('DataUpdater', {minute: minutes, hour: hours, dayOfWeek: weekday > 0 ? weekday : null}, () => {updating})
          setcronjob(job);
        })
        .catch(err => console.error("Cronjob not canceled: ", err))
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
          className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out"
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
        isOn={false}
      />
      <SettingComp
        title={t('obfuscated_mode')}
        short={t('obfuscated_mode_val')}
        icon={faUserNinja}
        isOn={true}
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