import Head from "next/head";
import { useRouter } from 'next/router';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHome } from "@fortawesome/free-solid-svg-icons";
import Image from "next/image";
import { useTranslation } from 'next-i18next';
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic';

/**
 * Function that generates the necessary static paths and props manually
 * This is to fix an issue with next18 translations
 */
const getStaticProps = makeStaticProps('common');
export { getStaticPaths, getStaticProps };

/**
 * This page is displayed after a scan if no virus was found, aka the system is "clean"
 * Displays a title, an image, and a button to navigate back to the home page.
 * @returns A full HTML page
 */
export default function Clean() {
    const router = useRouter();
    const { t } = useTranslation('common');

    /**
     * Function to navigate back to the home page
     */
    const backHome = () => {
        router.push('/');
    }

    return (
        <>
            <Head>
                {/* Set the page title using translated text */}
                <title>{t('clean_title')}</title>
            </Head>
            <div className="flex items-center justify-center h-screen flex-col">
                <h1 className="text-center mb-10 pt-4 font-medium ltext-5xl text-maingreen">
                    {/* Display the translated title */}
                    {t('clean_title')}
                </h1>
                <Image 
                    src="/images/success_image.png" 
                    alt="Success" 
                    className="max-w-[30%]" 
                    width={500}
                    height={500}
                />
                <button onClick={backHome} 
                    type="button" 
                    className="inline-block px-6 py-2.5 m-10 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md">
                    <FontAwesomeIcon
                        icon={faHome}
                        size="1x"
                        className="pr-1"
                    />
                    {/* Display the translated button text */}
                    {t('back_btn')}
                </button>
            </div>
        </>
    )
}
