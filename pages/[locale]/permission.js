import Head from "next/head";
import { useRouter } from "next/router";
import { faCheck, faXmark, faHome } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { useTranslation } from 'next-i18next';
import { getStaticPaths, makeStaticProps } from '../../lib/getStatic';

/**
 * Function that generates the necessary static paths and props manually
 * This is to fix an issue with next18 translations
 */
const getStaticProps = makeStaticProps('common')
export { getStaticPaths, getStaticProps }

/**
 * This page is a middle-page between the scanning process and the Home page. When the user requests a scan,
 * he/she gets redirected here, where a confirmation of th Agreement is required. Afterward, a scan can start.
 * @returns A full HTML page
 */
export default function Permission() {
  const router = useRouter();
  const {t} = useTranslation('common');

  // When the user accepts the agreement, a redirect takes place with the path to scan retrieved from the Home page
  function startScanner() {
    let { query: { scan_path , hashcount}, } = router;
    router.push({
      pathname: '/loading',
      query: { scan_path: scan_path, hashcount: hashcount }
    })
  }

  // Function to return home
  const backHome = () => {
    router.push("/");
  };

  return (
    <>
      <Head>
        <title>{t('permissions_title')}</title>
      </Head>
      <div className="align-middle">
        <button
          onClick={backHome}
          type="button"
          className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md"
        >
          <FontAwesomeIcon
              icon={faHome}
              size="1x"
              className="pr-1"
            /> 
          {t('back_btn')}
        </button>
        <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
        {t('permissions_title')}
        </h1>
      </div>
      <div className="flex items-center justify-center flex-col">
        <div className="m-2 overflow-y-scroll h-64">
          <p className="p-2 font-small leading-tight bg-gray-300 shadow-inner">
          {t('permissions_text')}
          </p>
        </div>

        <div className="align-middle">
          <button
            onClick={backHome}
            type="button"
            className="inline-block px-6 py-2.5 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md"
          >
            <FontAwesomeIcon
              icon={faXmark}
              style={{ fontSize: "1.3em" }}
              className="pr-1"
            />
            {t('perms_decline')}
          </button>
          <button
            onClick={startScanner}
            type="button"
            className="inline-block px-6 py-2.5 m-3 bg-maingreen text-white font-medium text-xs leading-tight uppercase rounded shadow-md"
          >
            <FontAwesomeIcon
              icon={faCheck}
              style={{ fontSize: "1.3em" }}
              className="pr-1"
            />
            {t('perms_accept')}
          </button>
        </div>
      </div>
    </>
  );
}
