import Head from "next/head";
import styles from "../../styles/refresh.module.css";
import { useState, useEffect } from "react";
import { useRouter } from "next/router";
import { invoke } from "@tauri-apps/api/tauri";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faGear } from '@fortawesome/free-solid-svg-icons';
import Swal from "sweetalert2";
import Image from "next/image";
import { useLocalStorage } from "../../services/useLocalStorage";
import DirectoryPickerButton from "../../components/DirectoryPickerButton";
import Dropdown from "../../components/DriverDropdown";
import DirectoryInput from "../../components/DirectoryInput";
import SwitchLanguage from "../../components/LanguagePicker";
import { useTranslation } from 'next-i18next'
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic'

const getStaticProps = makeStaticProps('common')
export { getStaticPaths, getStaticProps }

export default function Home() {
  const router = useRouter();
  const [value, setValue] = useState("None");
  const [dictionary, setDictionary] = useState([]);
  const [isRaspberryPi, setIsRaspberryPi] = useState(false);
  const [selectedDirectory, setSelectedDirectory] = useState(false);
  const {t} = useTranslation('common');

  const handleSelectDirectory = (directory) => {
    console.log("Incoming dir: ", directory);
    setValue(directory);
    setSelectedDirectory(true)
  }

  let {
    query: { scanner_error },
  } = router;

  const [errorOccurred, setError] = useLocalStorage("errorOccurred", 'false');
  if (scanner_error != null && scanner_error != "" && errorOccurred == 'true') {
    console.error("Home error", scanner_error);
    Swal.fire(t('scanning_error'), scanner_error, "error");
    setError('false');
    localStorage.removeItem("errorOccurred");
  }

  useEffect(() => {
    if (typeof window !== "undefined") {

      invoke("check_raspberry", {})
        .then((output) => setIsRaspberryPi(output))

      invoke("list_usb_drives", {})
        .then((output) => {
          setDictionary(JSON.parse(output));
        })
        .catch((error) => {
          console.error(error);
          Swal.fire(
            t('usb_list_error'),
            t('usb_list_error_msg'),
            "error"
          );
        });
    }
  }, [t]);

  const openAgreement = () => {
    if (value.length <= 0 || value == "None") {
      Swal.fire(t('selection_warn'), t('selection_warn_msg'), "warning");
    } else {
      router.push({
        pathname: "/permission",
        query: { scan_path: value },
      });
    }
  };

  const openInfo = () => {
    router.push("/info");
  };

  const openSettings = () => {
    router.push("/settings");
  };

  function refreshContent() {
    let refreshButton = document.getElementById("refresh-icon");
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
          <button
            onClick={openSettings}
            type="button"
            className="absolute top-0 right-0 px-6 py-2 border-2 m-2 border-maingreen text-maingreen bg-white 
        font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 
        focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
          >
            <FontAwesomeIcon
              icon={faGear}
              size="1x"
              className="pr-1"
            />
            {t('settings')}
          </button>
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
                {!isRaspberryPi && <DirectoryPickerButton onSelectDirectory={handleSelectDirectory} />}

                <button
                  onClick={refreshContent}
                  className="inline-block p-3 ml-1 bg-maingreen rounded shadow-md hover:bg-maingreen-dark hover:shadow-lg focus:bg-maingreen-dark focus:shadow-lg focus:outline-none focus:ring-0 active:maingreen-dark active:shadow-lg transition duration-150 ease-in-out"
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
                  className="mr-2 inline-block px-7 py-3 border-2 border-maingreen text-maingreen bg-white font-medium text-sm leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
                >
                  {t('info')}
                </button>
                <button
                  onClick={openAgreement}
                  type="button"
                  className="ml-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:mainred-dark active:shadow-lg transition duration-150 ease-in-out"
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
