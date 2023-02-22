import Head from "next/head"
import VirusComp from "../components/virus-comp"
import { useRouter } from "next/router";
import { useContext } from 'react';
import { SettingsContext } from '../state/context';

export default function Infected() {
    const router = useRouter();
    const { settings } = useContext(SettingsContext);
    const obfuscatedMode = settings["ObfuscatedMode"] != undefined ? settings["ObfuscatedMode"] : true;
    let { query: { virus_list }, } = router;
    console.log(virus_list);

    const backHome = () => {
        router.push('/');
    }

    if (obfuscatedMode) {
        return (
            <>
                <Head>
                    <title>Virus found!</title>
                </Head>
                <div className="flex items-center justify-center h-full flex-col">
                    <h1 className="text-center mb-10 font-medium leading-tight text-5xl mt-0 text-mainred">Virus found!</h1>
                    <img src="/images/failure_image.png" alt="Failure" className="max-w-[30%]" />
                    <button onClick={backHome} type="button" className="inline-block px-6 py-2.5 m-10 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out">
                        <i className="pr-1 fa fa-home"></i>
                        Home
                    </button>
                </div>
            </>
        )
    }
    return (
        <>
            <Head>
                <title>USB infected</title>
            </Head>
            <div className="align-middle">
                <button onClick={backHome} type="button" className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-bmainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out">
                    <i className="pr-1 fa fa-home"></i>
                    Home
                </button>
                <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">Virus found!</h1>
            </div>

            <div className="m-8 relative">
                {Array.isArray(virus_list) 
                    && virus_list.length > 0 
                    && virus_list.map(
                        entry => 
                            <VirusComp 
                                title={(entry.split('\\').pop().split('/').pop().split('.'))[0]} 
                                text={entry}/>
                    )
                }
            </div>
        </>
    )
}