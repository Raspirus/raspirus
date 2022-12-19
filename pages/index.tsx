import Head from 'next/head'
import styles from '../styles/main_page.module.css'
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
        <div className={styles.header_container}>
          <button className={styles.settings_btn}>SETTINGS <i className="fa fa-gear"></i></button>
        </div>
        <div className={styles.main_container}>
          <h1 className={styles.main_title}>RASPIRUS</h1>
          <div>
            <select className={styles.dropdown_selector}>
              <option value="Drive1">Drive1</option>
              <option value="Drive2">Drive2</option>
              <option value="Drive3">Drive3</option>
              <option value="Drive4">Drive4</option>
            </select>
          </div>
          <div>
            <button className={styles.start_btn}>START</button>
            <button className={styles.info_btn}>INFO</button>
          </div>
        </div>
      </main>
    </>
  )
}
