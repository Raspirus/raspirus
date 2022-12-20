import Head from 'next/head'
import { invoke } from "@tauri-apps/api/tauri"

// Note: When working with Next.js in development you have 2 execution contexts:
// - The server (nodejs), where Tauri cannot be reached, because the current context is inside of nodejs.
// - The client (webview), where it is possible to interact with the Tauri rust backend.
// To check if we are currently executing in the client context, we can check the type of the window object;
const isClient = typeof window !== 'undefined'

// Now we can call our Command!
// Right-click on the application background and open the developer tools.
// You will see "Hello, World!" printed in the console.
isClient && invoke('greet', { name: 'World' }).then(console.log).catch(console.error)

export default function Home() {
  return (
    <>
      <Head>
        <title>Raspirus</title>
        <meta name="description" content="Raspirus TAURI frontend" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <div className=''>
          <button type="button" className="inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 
        font-medium text-xs leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 
        focus:outline-none focus:ring-0 transition duration-150 ease-in-out"><i className="fa fa-gear"></i> SETTINGS</button>
        </div>


        <div className="p-12 text-center relative rounded-lg">
          <div
            className="absolute top-0 right-0 bottom-0 left-0 w-full h-full bg-fixed">
            <div className="flex justify-center items-center h-full">
              <div className="text-white">
                <h1 className="font-medium leading-tight text-5xl mt-0 mb-2 text-blue-600">RASPIRUS</h1>
                    <select defaultValue="0" className="form-select appearance-none
                        block
                        w-full
                        px-3
                        py-1.5
                        text-base
                        font-normal
                        text-gray-700
                        bg-white bg-clip-padding bg-no-repeat
                        border border-solid border-gray-300
                        rounded
                        transition
                        ease-in-out
                        m-0
                        focus:text-gray-700 focus:bg-white focus:border-blue-600 focus:outline-none" aria-label="Default select example">
                      <option value="0">Select your drive</option>
                      <option value="1">Drive 1</option>
                      <option value="2">Drive 2</option>
                      <option value="3">Drive 3</option>
                    </select>
                  <button type="button" className="inline-block px-7 py-3 bg-blue-600 text-white font-medium text-sm leading-snug uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out">START</button>
                  <button type="button" className="inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-sm leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 transition duration-150 ease-in-out">INFO</button>
              </div>
            </div>
          </div>
        </div>
      </main>
    </>
  )
}
