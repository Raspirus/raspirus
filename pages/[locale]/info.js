import Head from "next/head";
import { useRouter } from "next/router";
import InfoComp from "../../components/InformationCard";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faR, faBook, faUser, faInfoCircle, faScaleBalanced, faHome } from '@fortawesome/free-solid-svg-icons';
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
 * A page similar to the settings page that diaplays a list of cards.
 * The cards are not interactive, but purely for style and display valuable information
 * @returns A full HTML page
 */
export default function Info() {
    const router = useRouter();
    // Retrieves the current version set in the package.json
    const appVersion = require('../../package.json').version;
    const {t} = useTranslation('common');

    // Button to return Home
    const backHome = () => {
        router.push('/');
    }

    return (
        <>
            <Head>
                <title>{t('info_title')}</title>
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
                <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
                    {t('info_title')}
                </h1>
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