import Head from 'next/head';
import SettingComp from '../components/settings-comp';
import { useRouter } from 'next/router';
import { faFileLines, faUserNinja, faTerminal, faWrench } from '@fortawesome/free-solid-svg-icons';

export default function Settings() {
  const router = useRouter();

  const backHome = () => {
    router.push('/');
  };

  return (
    <>
      <Head>
        <title>Settings</title>
      </Head>
      <div className="align-middle">
        <button
          onClick={backHome}
          type="button"
          className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out"
        >
          <i className="pr-1 fa fa-home"></i>
          Home
        </button>
        <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
          Settings
        </h1>
      </div>

      <SettingComp
        title="Update Database"
        short="Updates the database (requires an internet connection)"
        icon={faWrench}
        isOn={false}
      />
      <SettingComp
        title="Activate Logging"
        short="Activates the writing of logs"
        icon={faFileLines}
        isOn={false}
      />
      <SettingComp
        title="Obfuscated Mode"
        short="When ON, it will not display the Path of possible found viruses"
        icon={faUserNinja}
        isOn={true}
      />

    </>
  );
}