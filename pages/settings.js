import Head from 'next/head';
import SettingComp from '../components/settings-comp';
import { useRouter } from 'next/router';
import { invoke } from "@tauri-apps/api/tauri";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faFileLines, faUserNinja, faWrench, faHome } from '@fortawesome/free-solid-svg-icons';
import { useState } from 'react';
import Swal from 'sweetalert2';
import moment from "moment";
import useTranslation from 'next-translate/useTranslation';

export default function Settings() {
  const router = useRouter();
  const t = useTranslation('common').t;
  const [hash_count, setCount] = useState(0);
  const [updated_date, setDate] = useState(t('update_db_status_1'));
  let db_location = "";

  const backHome = () => {
    router.push('/');
  };

  const updating = () => {
    if (typeof window !== "undefined") {
      Swal.fire({
        title: t('update_db_loading'),
        text: t('update_db_loading_val'),
        iconHtml: '<img src=images/loading-anim.gif>',
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

      <div className="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
            <div className="flex items-center justify-between mx-4">
                <div className="flex items-center">
                    <FontAwesomeIcon
                        icon={faWrench}
                        size="2x"
                        className="w-16 h-16 rounded-2xl p-3 border border-maingreen-light text-maingreen-light bg-green-50"
                    />
                    <div className="flex flex-col ml-3">
                        <div className="font-medium leading-none">{t('update_db')}</div>
                        <p className="text-sm text-gray-600 leading-none mt-1">{t('update_db_val')}</p>
                        <p className="text-sm text-gray-600 leading-none mt-1"><b>{t('update_db_1')}:</b> {hash_count} | <b>{t('update_db_2')}:</b> {updated_date}</p>
                    </div>
                </div>
                <button
                    onClick={updating}
                    className={`flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 text-white rounded-full bg-blue-500 border-blue-500`}>
                    {t('update_db_btn')}
                </button>
            </div>
        </div>

      <SettingComp
        title= {t('activate_logs')}
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

    </>
  );
}