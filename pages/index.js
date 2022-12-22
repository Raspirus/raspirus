import Head from 'next/head'
import { useRouter } from 'next/router';
import { useState } from 'react';
import { invoke } from "@tauri-apps/api/tauri"

export default function Home() {
  const router = useRouter();
  const [value, setValue] = useState("drive_0");
  let drives_list = null;
  invoke('list_usb_drives')
    .then((message) => drives_list = message)
    .catch(console.error);

  console.log("Drives: " + drives_list);


  const startScanner = async () => {

    router.push('/loading');

    console.log("Value = " + value);

    // To check if we are currently executing in the client context, we can check the type of the window object;
    const isClient = typeof window !== 'undefined';
    let dirty_array = null;
    let scanning_path = "/";
    let should_update = false;
    let db_location = "";

    // Now we can call our Command!
    // Right-click on the application background and open the developer tools.
    // You will see "Hello, World!" printed in the console.
    if (isClient) {
      await invoke('start_scanner', { path: scanning_path, update: should_update, dbfile: db_location })
        .then((message) => dirty_array = message)
        .catch((console.error));
    } else {
      console.error("Nextjs not in client mode!");
    }
    console.log(dirty_array);

    if (dirty_array != null && dirty_array.length > 0) {
      router.push('/infected')
    } else {
      router.push('/clean');
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
          <button onClick={openSettings} type="button" className="inline-block px-6 py-2 border-2 m-5 border-blue-600 text-blue-600 
        font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 
        focus:outline-none focus:ring-0 transition duration-150 ease-in-out"><i className="fa fa-gear"></i> SETTINGS</button>
        </div>


        <div className="p-12 text-center relative rounded-lg">
          <div>
            <div className="flex justify-center items-center h-full">
              <div className="text-white">
                <h1 className="font-medium leading-tight text-5xl mt-0 mb-2 text-blue-600">RASPIRUS</h1>
                <select value={value} onChange={(e) => {setValue(e.target.value);}} className="form-select appearance-none
                        block w-full px-3 py-1.5 text-base font-normal text-gray-700 bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300 rounded transition ease-in-out m-0
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none" aria-label="Default select example">
                  <option value="drive_0">Select your drive</option>
                  <option value="C:\Users\benbe\Documents\Coding\PyProjects\Testing">Test this</option>
                  <option value="drive_2">Drive 2</option>
                  <option value="drive_3">Drive 3</option>
                </select>
                <div className="mt-5">
                  <button onClick={startScanner} type="button" className="mr-2 inline-block px-7 py-3 bg-blue-600 text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out">START</button>
                  <button onClick={openInfo} type="button" className="ml-2 inline-block px-7 py-3 border-2 border-blue-600 text-blue-600 font-medium text-sm leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out">INFO</button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </>
  )
}
