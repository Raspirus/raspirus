import Head from "next/head"
import SettingComp from "../components/settings-comp"
import { useRouter } from 'next/router';
import { useState } from 'react';

export default function Settings() {
    const router = useRouter();

    const [logColor, setLogColor] = useState("red");
    const [activeText, setActive] = useState("OFF");

    const backHome = () => {
        router.push('/');
      }
    
    const updateDB = () => {
        // Update database
    }

    const activateLogging = () => {
        // Activate logging and change color
        if (logColor == "green") {
            setActive("OFF");
            setLogColor("red");
        } else {
            setLogColor("green");
            setActive("ON");
        }
    }

    const activateSSH = () => {
        // Activate SSH and change color
    }

    const activateFTP = () => {
        // Activate SSH and change color
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
                clicked={updateDB}
            />
            <SettingComp 
                title="Activate Logging"
                short="Activates bdebugging and logging"
                color={logColor}
                action={activeText}
                clicked={activateLogging}
            />
            <SettingComp 
                title="Activate FTP"
                short="Activates the File Transfer Protocol on the Raspberry Pi"
                color="yellow"
                action="WIP"
                clicked={activateFTP}
            />

            <SettingComp 
                title="Activate SSH"
                short="Activates the SSH Protocol for remote control"
                color="yellow"
                action="WIP"
                clicked={activateSSH}
            />
        </>
    )
}