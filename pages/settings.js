import Head from "next/head"
import SettingComp from "../components/settings-comp"
import { useRouter } from 'next/router';

export default function Settings() {
    const router = useRouter();

    const backHome = () => {
        router.push('/');
      }

    return (
        <>
            <Head>
                <title>Settings</title>
            </Head>
            <div>
                <button onClick={backHome} type="button" className="inline-block px-6 py-2.5 m-2 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out">
                    <i className="fa fa-home"></i>
                    Home
                </button>
            </div>
            <SettingComp 
                title="Update database"
                short="Updates the database (requires an internet connection)"
                color="blue"
                action="UPDATE"
            />
            <SettingComp 
                title="Activate Logging"
                short="Activates bdebugging and logging"
                color="green"
                action="ACTIVE"
            />
            <SettingComp 
                title="Activate FTP"
                short="Activates the File Transfer Protocol on the Raspberry Pi"
                color="yellow"
                action="WIP"
            />

            <SettingComp 
                title="Activate SSH"
                short="Activates the SSH Protocol for remote control"
                color="yellow"
                action="WIP"
            />
        </>
    )
}