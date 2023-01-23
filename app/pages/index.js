import Head from 'next/head'
import { useRouter } from 'next/router';
import { useState, useEffect, useContext } from 'react';
import { invoke } from "@tauri-apps/api/tauri";
import { alertService } from '../services/alert.service';
import { SettingsContext } from '../state/context';

export default function Home() {
  const router = useRouter();
  const [value, setValue] = useState("None");
  const [dictionary, setDictionary] = useState([]);

  const { settings } = useContext(SettingsContext);

  // const updateDatabase = settings.hasOwnProperty("Update database") ? settings["Update database"] : false;

  console.log("Settings = ", settings);
  console.log("Update DB = ", settings["UpdateDatabase"]);
  console.log("Activate Logs = ", settings["ActivateLogging"]);

  // Settings =  {Update database: true, Activate Logging: undefined, Activate FTP: undefined, Activate SSH: undefined}

  let { query: { data }, } = router;
  if (data != null && data != "") {
    console.error("Home error", data);
    alertService.error("Scanning failed: " + data);
  }

  if (typeof window !== 'undefined') {
  
    useEffect(() => {
      invoke('list_usb_drives', {})
      .then(output => {
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
        pathname: '/permission',
        query: { scan_path: value }
      })
    }
  }

  const openInfo = () => {
    router.push('/info');
  }

  const openSettings = () => {
    router.push('/settings');
  }


  return (
    <>
      <Head>
        <title>Raspirus</title>
      </Head>
      <main className='overflow-y-hidden'>
        <div className='flex justify-end'>
          <button onClick={openSettings} type="button" className="inline-block px-6 py-2 border-2 m-5 border-mainred text-mainred 
        font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 
        focus:outline-none focus:ring-0 transition duration-150 ease-in-out">
            <i className="pr-1 fa fa-gear"></i> SETTINGS</button>
        </div>


        <div className="p-12 text-center relative rounded-lg">
          <div>
            <div className="flex justify-center items-center h-full">
              <div className="w-full">
                <h1 className="font-bold leading-tight text-8xl mt-0 mb-2 text-mainred">RASPIRUS</h1>
                {Array.isArray(dictionary) && dictionary.length > 0 ? (
                <select placeholder='Select drive' value={value} 
                  onChange={(e) => { console.log("Changed drive: " + e.target.value); setValue(e.target.value); }} 
                  className="
                        px-3 py-1.5 text-base font-normal text-gray-700 bg-white w-9/12
                        border border-solid border-maingreen-light rounded transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none">
                    <option value="None">Select your driver</option>
                  {dictionary.map((item, i) => (
                    <option key={i} value={item.path}>{item.name}</option>
                  ))}
                </select>
                ) : (
                  <div className="
                  m-auto px-3 py-1.5 text-base font-normal text-gray-700 bg-white w-9/12
                  border border-solid border-maingreen-light rounded transition ease-in-out
                  focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none">
                    No drives found. Insert a drive and refresh this page</div>
                )}
                <div className="mt-5">
                  <button onClick={openAgreement} type="button" className="mr-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:mainred-dark active:shadow-lg transition duration-150 ease-in-out">START</button>
                  <button onClick={openInfo} type="button" className="ml-2 inline-block px-7 py-3 border-2 border-mainred text-mainred font-medium text-sm leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out">INFO</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </>
  )
}
