import Head from "next/head";
import styles from "../../styles/refresh.module.css";
import { useState, useEffect } from "react";
import { useRouter } from "next/router";
import { invoke } from "@tauri-apps/api/tauri";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faGear, faWrench } from '@fortawesome/free-solid-svg-icons';
import Swal from "sweetalert2";
import Image from "next/image";
import { useLocalStorage } from "../../services/useLocalStorage";
import DirectoryPickerButton from "../../components/DirectoryPickerButton";
import Dropdown from "../../components/DriverDropdown";
import DirectoryInput from "../../components/DirectoryInput";
import SwitchLanguage from "../../components/LanguagePicker";
import { useTranslation } from 'next-i18next';
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic';

/**
 * Function that generates the necessary static paths and props manually
 * This is to fix an issue with next18 translations
 */
const getStaticProps = makeStaticProps('common')
export { getStaticPaths, getStaticProps }

/**
 * The main page of the application, displaying buttons to visit both the Information as the Settings page
 * It also determines through the backend on which architecture the app is running to hide certain buttons
 * Mainly it allows the user to choose a device or location to scan and start the scan.
 * @returns A full HTML page
 */
export default function Home() {
  const router = useRouter();
  // Keeps the selected directory, if one was selected
  const [value, setValue] = useState("None");
  // Array of scannable USB devices
  const [dictionary, setDictionary] = useState([]);
  // If the app is running on the Raspberry Pi (to disallow scanning folders)
  const [isRaspberryPi, setIsRaspberryPi] = useState(false);
  // If a directory was selected
  const [selectedDirectory, setSelectedDirectory] = useState(false);
  // Whether the dialog is a directory selection or not.
  const [dir_selection, SetDirSelection] = useState(false);
  // Determines if an error occurred
  const [errorOccurred, setError] = useLocalStorage("errorOccurred", 'false');
  const { t } = useTranslation('common');
  const [updateAvailable, setUpdateAvailable] = useState(false);

  /**
   * Function triggered when a directory was selected
   * @param {Path} directory Path to that directory as a string
   */
  const handleSelectDirectory = (directory) => {
    setValue(directory);
    setSelectedDirectory(true)
  }

  // Reads the query params to see if a previous running process returned an error.
  // For example if scanning failed, the user is redirected back to the Home page
  // and an error message is appended as a query parameter, which is extracted here
  let {
    query: { scanner_error },
  } = router;


  if (scanner_error != null && scanner_error != "" && errorOccurred == 'true') {
    console.error("Home error", scanner_error);
    // Fires a JavaScript Alert to inform the user that an issue occurred
    Swal.fire(t('scanning_error'), scanner_error, "error");
    // Then removes the error again, to avoid firing the alert twice
    setError('false');
    localStorage.removeItem("errorOccurred");
  }

  useEffect(() => {
    if (typeof window !== "undefined") {
      // Using the backend, check if a config file exists and load it
      invoke("load_config_fe", {})
        .then((output) => {
          const parsedData = JSON.parse(output);
          console.log("Loaded config: ", parsedData);
          SetDirSelection(parsedData.scan_dir);
          if (parsedData.last_db_update == "Never") {
            Swal.fire({
              title: t('welcome'),
              text: t('welcome_text'),
              icon: "info",
              footer: t('welcome_footer')
            });
          }
        })
        .catch((error) => {
          SetDirSelection(true);
          console.error("Couldn't retrieve settings: ", error);
        })

      // Using the backend, check if the app is running on the Raspberry Pi
      invoke("check_raspberry", {})
        .then((output) => setIsRaspberryPi(output))

      // Using the backend, ask for a list of connected USB drives
      invoke("list_usb_drives", {})
        .then((output) => {
          setDictionary(JSON.parse(output));
        })
        .catch((error) => {
          // Catches possible errors and display them using an Alert
          console.error(error);
          Swal.fire({
            title: t('usb_list_error'),
            text: t('usb_list_error_msg'),
            icon: "error"
          });
        });

      // Using the backend, check if an update is available
      invoke("check_update", {})
        .then((output) => {
          setUpdateAvailable(output);
        })
        .catch((error) => {
          // Web request not successful, probably no internet connection
          console.error(error);
        });
    }
  }, [t]);

  /**
   * The user is not directly redirected to the scanning process, he/she first needs to pass
   * and accept the agreement page. Once done that, the scanning will start from that page.
   * In this case we simply redirect the user to the agreement page and pass on some values
   * by attaching them to the URL query.
   */
  const openAgreement = () => {
    // Checks that a folder or device was selected
    if (value.length <= 0 || value == "None") {
      Swal.fire(t('selection_warn'), t('selection_warn_msg'), "warning");
    } else {
      router.push({
        pathname: "/permission",
        query: { scan_path: value },
      });
    }
  };

  // Redirects the user to the Information page
  const openInfo = () => {
    router.push("/info");
  };

  // Redirects the user to the Settings page
  const openSettings = () => {
    router.push("/settings");
  };

  /**
   * Manually refreshes the page to reload the attached USB drives.
   * This measure is necessary and was choosen over the constant-ping method.
   * Instead of asking the backed every X time to send us data, we let the user decide
   * when he inserted a new thum drive.
   */
  function refreshContent() {
    let refreshButton = document.getElementById("refresh-icon");
    // Starts a small animation
    refreshButton.classList.add(styles.refreshStart);

    if (typeof window !== "undefined") {
      invoke("list_usb_drives", {})
        .then((output) => {
          setDictionary(JSON.parse(output));
          setTimeout(() => {
            refreshButton.classList.remove(styles.refreshStart);
          }, 3000);
        })
        .catch((error) => {
          console.error(error);
          refreshButton.classList.remove(styles.refreshStart);
          Swal.fire(
            t('usb_list_error'),
            t('usb_list_error_msg'),
            "error"
          );
        });
    }
  }

  return (
    <>
      <Head>
        <title>{t('title')}</title>
      </Head>
      <main className="h-screen">
        <div className="flex justify-start">
          <SwitchLanguage />

          <div className="flex justify-center absolute top-0 right-0">

            {updateAvailable && (
              <button
                onClick={() => router.push("/settings")}
                type="button"
                className="px-2 py-2 border-2 m-2 border-mainred text-white bg-mainred 
      font-medium text-xs leading-tight uppercase rounded"
              >
                <FontAwesomeIcon
                  icon={faWrench}
                  size="1x"
                  className="pr-1"
                />
                {t('db_update_notif')}
              </button>
            )}

            <button
              onClick={openSettings}
              type="button"
              className="px-6 py-2 border-2 m-2 border-maingreen text-maingreen bg-white 
        font-medium text-xs leading-tight uppercase rounded"
            >
              <FontAwesomeIcon
                icon={faGear}
                size="1x"
                className="pr-1"
              />
              {t('settings')}
            </button>

          </div>
        </div>

        <div className="flex h-full justify-center p-12 text-center">
          <div className="flex justify-center items-center h-full">
            <div className="w-full">
              <h1 className="font-bold leading-tight text-8xl mt-0 mb-2 text-mainred uppercase">
                {t('title')}
              </h1>

              <div className="flex justify-center">
                {selectedDirectory ?
                  <DirectoryInput value={value} />
                  : <Dropdown dictionary={dictionary} value={value} setValue={setValue} />
                }
                {!isRaspberryPi && <DirectoryPickerButton onSelectDirectory={handleSelectDirectory} scanDirectory={dir_selection} />}

                <button
                  onClick={refreshContent}
                  className="inline-block p-3 ml-1 bg-maingreen rounded shadow-md"
                >
                  <Image
                    id="refresh-icon"
                    className="h-full w-4"
                    src="images/refresh.svg"
                    alt="Refresh"
                    width={500}
                    height={500}
                  />
                </button>
              </div>
              <div className="mt-2">
                <button
                  onClick={openInfo}
                  type="button"
                  className="mr-2 inline-block px-7 py-3 border-2 border-maingreen text-maingreen bg-white font-medium text-sm uppercase rounded"
                >
                  {t('info')}
                </button>
                <button
                  onClick={openAgreement}
                  type="button"
                  className="ml-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm uppercase rounded shadow-md"
                >
                  {t('start')}
                </button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </>
  );
}
