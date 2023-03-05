import Head from 'next/head';
import SettingComp from '../components/settings-comp';
import { useRouter } from 'next/router';
import { invoke } from "@tauri-apps/api/tauri";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faFileLines, faUserNinja, faWrench } from '@fortawesome/free-solid-svg-icons';
import Swal from 'sweetalert2';

export default function Settings() {
  const router = useRouter();
  let db_location = "";

  const backHome = () => {
    router.push('/');
  };

  const updating = () => {
    if (typeof window !== "undefined") {
      Swal.fire({
        title: 'Updating database...',
        text: 'Please be patient, this can take some time',
        iconHtml: '<img src=images/loading-anim.gif>',
        allowOutsideClick: false,
        showConfirmButton: false,
        allowEscapeKey: false,
        allowEnterKey: false
      })
      invoke("update_database", {
        dbfile: db_location,
      })
        .then((message) => {
          console.log(message);
          Swal.fire("Update completed", "Database is up-to-date", "success");
        })
        .catch((error) => {
          console.error(error);
          Swal.fire("Update error", "Couldn't start the update", "error");
        });
    } else {
      console.error("Nextjs not in client mode!");
      Swal.fire("Window error", "Nextjs is not in client mode", "error");
    }
  }

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

      <div className="flex flex-col m-6 p-2 bg-white rounded-2xl shadow-md">
            <div className="flex items-center justify-between mx-4">
                <div className="flex items-center">
                    <FontAwesomeIcon
                        icon={faWrench}
                        size="2x"
                        className="w-16 h-16 rounded-2xl p-3 border border-red-100 text-red-400 bg-red-50"
                    />
                    <div className="flex flex-col ml-3">
                        <div className="font-medium leading-none">Update Database</div>
                        <p className="text-sm text-gray-600 leading-none mt-1">Updates the database (requires an internet connection)</p>
                    </div>
                </div>
                <button
                    onClick={updating}
                    className={`flex-no-shrink px-5 ml-4 py-2 text-sm shadow-sm hover:shadow-lg font-medium tracking-wider border-2 text-white rounded-full bg-blue-500 border-blue-500`}>
                    UPDATE
                </button>
            </div>
        </div>

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