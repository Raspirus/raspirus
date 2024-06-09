import Head from "next/head";
import VirusComp from "../../components/VirusCard";
import { invoke } from "core";
import { useRouter } from "next/router";
import { useState, useEffect } from 'react';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHome } from "@fortawesome/free-solid-svg-icons";
import Image from "next/image";
import { useTranslation } from 'next-i18next';
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic';

/**
 * Function that generates the necessary static paths and props manually
 * This is to fix an issue with next18 translations
 */
const getStaticProps = makeStaticProps('common')
export { getStaticPaths, getStaticProps }

/**
 * This page shows either a list of found viruses or, if obfuscated mode is active, just a warning
 * @returns A full HTML page
 */
export default function Infected() {
    const router = useRouter();
    // Check if obfuscated mode is active
    const [obfuscated, setObfuscated] = useState(false);
    let { query: { virus_list }, } = router;
    const { t } = useTranslation('common');

    if (typeof virus_list === "string") {
        virus_list = JSON.parse(virus_list);
    }

    // Button to return back Home
    const backHome = () => {
        router.push('/');
    }

    useEffect(() => {
        if (typeof window !== "undefined") {

            // We retrieve the set obfuscated mode from the backends config
            invoke("load_config_fe", {})
                .then((output) => {
                    const parsedData = JSON.parse(output);
                    console.log("Parsed Config data: ", parsedData);
                    setObfuscated(parsedData.obfuscated_is_active);
                })
                .catch((err) => console.error(err))
        }
    }, []);

    if (obfuscated) {
        return (
            <>
                <Head>
                    <title>{t('infected_title')}</title>
                </Head>
                <div className="flex items-center justify-center h-screen flex-col">
                    <h1 className="text-center mb-10 pt-4 font-medium leading-tight text-5xl mt-0 text-mainred">{t('infected_title')}</h1>
                    <Image
                        src="/images/failure_image.png"
                        alt="Failure"
                        className="max-w-[30%]"
                        width={500}
                        height={500}
                    />
                    <button onClick={backHome} type="button" className="inline-block px-6 py-2.5 m-10 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md">
                        <FontAwesomeIcon
                            icon={faHome}
                            size="1x"
                            className="pr-1"
                        />
                        {t('back_btn')}
                    </button>
                </div>
            </>
        )
    }
    return (
        <>
            <Head>
                <title>{t('infected_title')}</title>
            </Head>
            <div className="align-middle">
                <button onClick={backHome} type="button" className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md">
                    <FontAwesomeIcon
                        icon={faHome}
                        size="1x"
                        className="pr-1"
                    />
                    {t('back_btn')}
                </button>
                <h1 className="inline-block align-middle p-2 pt-4 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">{t('infected_title')}</h1>
            </div>

            <div className="m-8 relative">
                {Array.isArray(virus_list) && virus_list.length > 0
                    ? virus_list.map((entry) => (
                        <VirusComp
                            key={entry}
                            title={(entry.split("\\").pop().split("/").pop().split("."))[0]}
                            text={entry}
                        />
                    ))
                    : [
                        <p key="error-message">
                            {t('infected_error')}
                        </p>,
                    ]}
            </div>
        </>
    )
}