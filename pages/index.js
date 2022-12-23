import Head from 'next/head'
import { useRouter } from 'next/router';
import { useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri"

export default function Home() {
  const router = useRouter();
  const [value, setValue] = useState("drive_0");
  let drives_list = null;

  if (typeof window !== 'undefined') {
    invoke('list_usb_drives')
    .then((message) => drives_list = message)
    .catch(console.error);

    console.log("Drives: " + drives_list);
  }

  async function startScanner() {
    router.push('/loading');
    console.log("Value = " + value);
    let dirty_array = null;
    let scanning_path = value;
    let should_update = false;
    let db_location = "";

    if (typeof window !== 'undefined') {
      await invoke('start_scanner', { path: scanning_path, update: should_update, dbfile: db_location })
        .then((message) => dirty_array = message)
        .catch((console.error));
        console.log(dirty_array);
        if (dirty_array != null && dirty_array.length > 0) {
          router.push('/infected');
        } else {
          router.push('/clean');
        }
    } else {
      console.error("Nextjs not in client mode!");
    }

    console.log("Finished");
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
      <main>
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
                <select placeholder='Select drive' value={value} onChange={(e) => {setValue(e.target.value);}} className="
                        px-3 py-1.5 text-base font-normal text-gray-700 bg-white w-9/12
                        border border-solid border-maingreen-light rounded transition ease-in-out
                        focus:text-gray-700 focus:bg-white focus:border-maingreen focus:outline-none">
                  <option value=".">This will scan the current folder</option>
                  <option value="/">This will scan the / folder</option>
                  <option value="C:/Users/benbe/Documents/School/Deutsch">Windows only</option>
                  <option value="/home/pi/FTP/">Linux PI only</option>
                </select>
                <div className="mt-5">
                  <button onClick={startScanner} type="button" className="mr-2 inline-block px-7 py-3 bg-mainred text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:mainred-dark active:shadow-lg transition duration-150 ease-in-out">START</button>
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
