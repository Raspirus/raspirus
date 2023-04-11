import Head from "next/head"
import { useRouter } from "next/router";
import InfoComp from "../components/info-comp";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faR, faBook, faUser, faInfoCircle, faScaleBalanced, faHome } from '@fortawesome/free-solid-svg-icons';
import Image from "next/image";
import useTranslation from 'next-translate/useTranslation';


export default function Info() {
    const router = useRouter();
    const appVersion = require('../package.json').version;
    const t = useTranslation('common').t;

    const backHome = () => {
        router.push('/');
    }

    return (
        <>
            <Head>
                <title>{t('info_title')}</title>
            </Head>
            <div className="align-middle">
                <button onClick={backHome} type="button" className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out">
                    <FontAwesomeIcon
                        icon={faHome}
                        size="1x"
                        className="pr-1"
                    />
                    {t('back_btn')}
                </button>
                <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">{t('info_title')}</h1>
            </div>

            <Image 
                src="/images/banner.png" 
                alt="Banner image"
                className="max-w-[90%] mx-auto rounded-xl shadow-md"
                width={1856}
                height={1024}
                />

            <InfoComp
                title={t('app_name')}
                value={t('title')}
                icon={faR}
            />
            <InfoComp
                title={t('description')}
                value={t('description_val')}
                icon={faBook}
            />
            <InfoComp
                title={t('maintainers')}
                value={t('maintainers_val')}
                icon={faUser}
            />
            <InfoComp
                title={t('version')}
                value={appVersion}
                icon={faInfoCircle}
            />
            <InfoComp
                title={t('license')}
                value={t('license_val')}
                icon={faScaleBalanced}
            />
        </>
    )
}