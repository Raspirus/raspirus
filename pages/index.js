import Head from "next/head";
import { useState, useEffect } from "react";
import { useRouter } from "next/router";
import { invoke } from "@tauri-apps/api/tauri";
import { alertService } from "../services/alert.service";

export default function Home() {
  const router = useRouter();
  const [value, setValue] = useState("None");
  const [dictionary, setDictionary] = useState([]);

  let {
    query: { scanner_error },
  } = router;
  if (scanner_error != null && scanner_error != "") {
    console.error("Home error", scanner_error);
    alertService.error("Scanning failed: " + scanner_error);
  }

  if (typeof window !== "undefined") {
    useEffect(() => {
      invoke("list_usb_drives", {})
        .then((output) => {
          console.log(JSON.parse(output));
          setDictionary(JSON.parse(output));
        })
        .catch(console.error);
    }, []);
  }

  const openAgreement = () => {
    console.log("Value selected = ", value);
    if (value.length <= 0 || value == "None") {
      alertService.warn("Please select a driver first!");
    } else {
      router.push({
        pathname: "/permission",
        query: { scan_path: value },
      });
    }
  };

  const openInfo = () => {
    // router.push('/info');
    router.push("/infected");
  };

  const openSettings = () => {
    router.push("/settings");
  };

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
            className="absolute top-0 right-0 px-6 py-2 border-2 m-5 border-mainred text-mainred bg-white 
        font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 
        focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
          >
            <i className="pr-1 fa fa-gear"></i> SETTINGS
          </button>
        </div>

        <div className="flex h-screen justify-center p-12 text-center">
          <div className="flex justify-center items-center h-full">
            <div className="w-full">
              <h1 className="font-bold leading-tight text-8xl mt-0 mb-2 text-mainred">
                RASPIRUS
              </h1>
              {Array.isArray(dictionary) && dictionary.length > 0 ? (
                <select
                  placeholder="Select drive"
                  value={value}
                  onChange={(e) => {
                    console.log("Changed drive: " + e.target.value);
                    setValue(e.target.value);
                  }}
                  className="
                        px-3 py-1.5 text-base font-normal text-gray-700 bg-white w-full
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
                  m-auto px-3 py-1.5 text-base font-normal text-gray-700 bg-white w-full
                  border border-solid border-maingreen-light rounded transition ease-in-out
                  focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none"
                >
                  No drives found. Insert a drive and refresh this page
                </div>
              )}
              <div className="mt-5">
                <button
                  onClick={openAgreement}
                  type="button"
                  className="mr-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:mainred-dark active:shadow-lg transition duration-150 ease-in-out"
                >
                  START
                </button>
                <button
                  onClick={openInfo}
                  type="button"
                  className="ml-2 inline-block px-7 py-3 border-2 border-mainred text-mainred bg-white font-medium text-sm leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out"
                >
                  INFO
                </button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </>
  );
}
