import Head from "next/head";
import { useRouter } from "next/router";
import { faCheck, faXmark, faHome } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export default function Permission() {
  const router = useRouter();

  function startScanner() {
    let { query: { scan_path }, } = router;
    router.push({
      pathname: '/loading',
      query: { scan_path: scan_path }
    })
  }

  const backHome = () => {
    router.push("/");
  };

  return (
    <>
      <Head>
        <title>No Virus found</title>
      </Head>
      <div className="align-middle">
        <button
          onClick={backHome}
          type="button"
          className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out"
        >
          <FontAwesomeIcon
              icon={faHome}
              size="1x"
              className="pr-1"
            /> 
          Back
        </button>
        <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
          User Agreement
        </h1>
      </div>
      <div className="flex items-center justify-center flex-col">
        <div className="m-2 overflow-y-scroll h-64">
          <p className="p-2 font-small leading-tight bg-gray-300 shadow-inner">
            This program is designed to scan for malicious software on USB drives that are inserted into your computer.
            By using this program, you acknowledge that you are responsible for the content of the USB drive and
            any potential damage it may cause to your computer system. You also acknowledge that the program is
            not guaranteed to detect all malware and cannot be held responsible for any malware that may go undetected.
            By inserting a USB drive into your computer and running the program,
            you are giving your consent to have the drive scanned for malware.
            If you do not consent to this, do not insert the USB drive or run the program.
            The results of the scan are for informational purposes only and should be carefully reviewed by the user.
            The program does not make any guarantees regarding the accuracy or completeness of the scan results.
            It is the user&apos;s responsibility to ensure that they are aware of any potential risks associated with using a USB drive,
            including the risk of malware infection. Users should only use USB drives from trusted sources and
            exercise caution when inserting unknown drives into their computer.
            The Raspirus program is provided on an &quot;as-is&quot; basis and cannot be held liable for any damages,
            including but not limited to, data loss, system crashes, or any other damages resulting from the use of the program.
            By using the Raspirus program, you agree to these terms and conditions.
            If you do not agree to these terms and conditions, do not use the program.
          </p>
        </div>

        <div className="align-middle">
          <button
            onClick={backHome}
            type="button"
            className="inline-block px-6 py-2.5 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out"
          >
            <FontAwesomeIcon
              icon={faXmark}
              style={{ fontSize: "1.3em" }}
              className="pr-1"
            />
            Decline
          </button>
          <button
            onClick={startScanner}
            type="button"
            className="inline-block px-6 py-2.5 m-3 bg-maingreen text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-maingreen-dark hover:shadow-lg focus:bg-maingreen-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-maingreen-dark active:shadow-lg transition duration-150 ease-in-out"
          >
            <FontAwesomeIcon
              icon={faCheck}
              style={{ fontSize: "1.3em" }}
              className="pr-1"
            />
            Accept
          </button>
        </div>
      </div>
    </>
  );
}
