import Head from "next/head"
import SettingComp from "../components/settings-comp"

export default function Settings() {
    return (
        <>
            <Head>
                <title>Settings</title>
            </Head>
            <div>
                <button>Test button</button>
            </div>
            <SettingComp 
                title="Update database"
                short="Updates the database (requires an internet connection)"
                action="UPDATE"
            />
            <SettingComp 
                title="Activate Logging"
                short="Activates bdebugging and logging"
                action="ACTIVE"
            />
            <SettingComp 
                title="Activate FTP"
                short="Activates the File Transfer Protocol on the Raspberry Pi"
                action="WIP"
            />

            <SettingComp 
                title="Activate SSH"
                short="Activates the SSH Protocol for remote control"
                action="WIP"
            />
        </>
    )
}