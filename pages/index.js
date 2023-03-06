import Head from "next/head";
import styles from "../styles/refresh.module.css";
import { useState, useEffect } from "react";
import { useRouter } from "next/router";
import { invoke } from "@tauri-apps/api/tauri";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faGear } from '@fortawesome/free-solid-svg-icons';
import Swal from "sweetalert2";
import Image from "next/image";
import { useLocalStorage } from "../services/useLocalStorage";

export default function Home() {
  const router = useRouter();
  const [value, setValue] = useState("None");
  const [dictionary, setDictionary] = useState([]);

  let {
    query: { scanner_error },
  } = router;

  const [errorOccurred, setError] = useLocalStorage("errorOccurred", 'false');
  console.log("Err: ", errorOccurred);
  if (scanner_error != null && scanner_error != "" && errorOccurred == 'true') {
    console.error("Home error", scanner_error);
    Swal.fire("Scanning errors", scanner_error, "error");
    console.log("LS = ", localStorage);
    setError('false');
    localStorage.removeItem("errorOccurred");
  }
  useEffect(() => {
    if (typeof window !== "undefined") {
      invoke("list_usb_drives", {})
        .then((output) => {
          console.log(JSON.parse(output));
          setDictionary(JSON.parse(output));
        })
        .catch((error) => {
          console.error(error);
          Swal.fire(
            "USB list error",
            "Couldn't search for USBs on this device",
            "error"
          );
        });
    }
  }, []);

  const openAgreement = () => {
    console.log("Value selected = ", value);
    if (value.length <= 0 || value == "None") {
      Swal.fire("No Selection", "Please select a driver first!", "warning");
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
          console.log(JSON.parse(output));
          setDictionary(JSON.parse(output));
          setTimeout(() => {
            refreshButton.classList.remove(styles.refreshStart);
          }, 3000);
        })
        .catch((error) => {
          console.error(error);
          refreshButton.classList.remove(styles.refreshStart);
          Swal.fire(
            "USB list error",
            "Couldn't search for USBs on this device",
            "error"
          );
        });
    }
  }

  return (
    <>
      <Head>
        <title>Raspirus</title>
      </Head>
      <main className="overflow-y-hidden">
        <div className="flex justify-end">
          <button
            onClick={openSettings}
            type="button"
            className="absolute top-0 right-0 px-6 py-2 border-2 m-5 border-maingreen text-maingreen bg-white 
        font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 
        focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
          >
            <FontAwesomeIcon
              icon={faGear}
              size="1x"
              className="pr-1"
            />
            SETTINGS
          </button>
        </div>

        <div className="flex h-screen justify-center p-12 text-center">
          <div className="flex justify-center items-center h-full">
            <div className="w-full">
              <h1 className="font-bold leading-tight text-8xl mt-0 mb-2 text-mainred">
                RASPIRUS
              </h1>

              <div className="flex justify-center">
                {Array.isArray(dictionary) && dictionary.length > 0 ? (
                  <select
                    placeholder="Select drive"
                    value={value}
                    onChange={(e) => {
                      console.log("Changed drive: " + e.target.value);
                      setValue(e.target.value);
                    }}
                    className="
                        px-3 py-1.5 text-base font-normal text-gray-700 bg-white inline-block mr-2 w-full
                        border border-solid border-maingreen-light rounded transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none"
                  >
                    <option value="None">Select your driver</option>
                    {dictionary.map((item, i) => (
                      <option key={i} value={item.path}>
                        {item.name}
                      </option>
                    ))}
                  </select>
                ) : (
                  <div
                    className="
                  m-auto px-3 py-1.5 text-base font-normal text-gray-700 bg-white inline-block w-full
                  border border-solid border-maingreen-light rounded transition ease-in-out mr-2
                  focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none"
                  >
                    No drives found. Insert a drive and refresh this page
                  </div>
                )}

                <button
                  onClick={refreshContent}
                  className="inline-block p-3 bg-maingreen rounded shadow-md hover:bg-maingreen-dark hover:shadow-lg focus:bg-maingreen-dark focus:shadow-lg focus:outline-none focus:ring-0 active:maingreen-dark active:shadow-lg transition duration-150 ease-in-out"
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
              <div className="mt-5">
                <button
                  onClick={openInfo}
                  type="button"
                  className="mr-2 inline-block px-7 py-3 border-2 border-maingreen text-maingreen bg-white font-medium text-sm leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
                >
                  INFO
                </button>
                <button
                  onClick={openAgreement}
                  type="button"
                  className="ml-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:mainred-dark active:shadow-lg transition duration-150 ease-in-out"
                >
                  START
                </button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </>
  );
}
