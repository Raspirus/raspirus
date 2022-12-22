import Head from "next/head"
import VirusComp from "../components/virus-comp"
import { useRouter } from "next/router";

export default function Infected() {
    const router = useRouter();

    const backHome = () => {
        router.push('/');
      }

    return (
        <>
            <Head>
                <title>USB infected</title>
            </Head>
            <div className="align-middle">
                <button onClick={backHome} type="button" className="inline-block align-middle px-6 py-2.5 m-2 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out">
                    <i className="pr-1 fa fa-home"></i>
                    Home
                </button>
                <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-red-600">Virus found!</h1>
            </div>

            <div className="m-8 relative">
                <VirusComp
                    title="Testtextcicsicbicökcbiwöcbiwlcbiwlcblwcbwlcvwlcbhwlucbwlucbwlcbwleucbwlcblwcbwluvulavulvlwubclwuvwulvluwv"
                    value="Testvalue"
                />
                <VirusComp
                    title="Testtextcicsicbicökcbiwöcbiwlcbiwlcblwcbwlcvwlcbhwlucbwlucbwlcbwleucbwlcblwcbwluvulavulvlwubclwuvwulvluwv"
                    value="Testvalue"
                />
                <VirusComp
                    title="Testtext"
                    value="Testvalue"
                />
                <VirusComp
                    title="Testtextcicsicbicökcbiwöcbiwlcbiwlcblwcbwlcvwlcbhwlucbwlucbwlcbwleucbwlcblwcbwluvulavulvlwubclwuvwulvluwv"
                    value="Testvalue"
                />
                <VirusComp
                    title="Testtext"
                    value="Testvalue"
                />

            </div>
        </>
    )
}