import Head from "next/head";
import { useRouter } from "next/router";
import { faCheck, faXmark } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

export default function Permission() {
  const router = useRouter();

  async function startScanner() {
    router.push("/loading");
    console.log("Value = " + value);
    let dirty_array = null;
    if (value == "") {
      return;
    }
    let scanning_path = value;
    let should_update = false;
    let db_location = "";

    if (typeof window !== "undefined") {
      await invoke("start_scanner", {
        path: scanning_path,
        update: should_update,
        dbfile: db_location,
      })
        .then((message) => (dirty_array = message))
        .catch(console.error);
      console.log(dirty_array);
      if (dirty_array != null && dirty_array.length > 0) {
        router.push("/infected");
      } else {
        router.push("/clean");
      }
    } else {
      console.error("Nextjs not in client mode!");
    }

    console.log("Finished");
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
          <i className="pr-1 fa fa-home"></i>
          Back
        </button>
        <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">
          User Agreement
        </h1>
      </div>
      <div className="flex items-center justify-center flex-col">
        <p className="m-5 p-2 font-small leading-tight bg-gray-300 shadow-inner">
          Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec at
          condimentum eros. Mauris lobortis dui et ex lobortis, a vulputate nisl
          maximus. Lorem ipsum dolor sit amet, consectetur adipiscing elit. In
          hac habitasse platea dictumst. Aliquam enim est, dignissim
          pellentesque lorem in, finibus rutrum risus. Donec imperdiet, felis
          vitae suscipit bibendum, erat dolor volutpat tortor, sed sagittis
          massa lorem quis ligula. Vestibulum porta sed dui vitae vehicula. Sed
          eleifend diam sit amet turpis gravida rhoncus. Aliquam congue nibh in
          mi auctor mollis. Donec pharetra cursus erat, in rhoncus sapien cursus
          vitae. Etiam maximus tortor sit amet elementum consectetur. Proin ut
          lorem lacus. Sed sodales, metus id tincidunt condimentum, erat est
          finibus lectus, ac consequat dolor felis in nulla. Sed mollis accumsan
          ipsum nec venenatis. Praesent quis nisl nisi. Proin accumsan
          pellentesque feugiat.
        </p>

        <div className="align-middle">
          <button
            onClick={backHome}
            type="button"
            className="inline-block px-6 py-2.5 m-5 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out"
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
            className="inline-block px-6 py-2.5 m-5 bg-maingreen text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-maingreen-dark hover:shadow-lg focus:bg-maingreen-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-maingreen-dark active:shadow-lg transition duration-150 ease-in-out"
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
