import Head from "next/head"
import { useRouter } from "next/router";
import InfoComp from "../components/info-comp"

export default function Info() {
    const router = useRouter();

    const backHome = () => {
        router.push('/');
    }

    return (
        <>
            <Head>
                <title>Information</title>
            </Head>
            <div className="align-middle">
                <button onClick={backHome} type="button" className="inline-block align-middle px-6 py-2.5 m-2 bg-blue-600 text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-blue-700 hover:shadow-lg focus:bg-blue-700 focus:shadow-lg focus:outline-none focus:ring-0 active:bg-blue-800 active:shadow-lg transition duration-150 ease-in-out">
                    <i className="fa fa-home"></i>
                    Home
                </button>
                <h1 class="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-blue-600">Information</h1>
            </div>
            <InfoComp
                title="App Name"
                value="Raspirus"
            />
            <InfoComp
                title="Description"
                value="Simple signatures-based antivirus for single-board computers like Raspbrry Pi"
            />
            <InfoComp
                title="Contributors"
                value="Demetz Benjamin, Hell Björn Felix"
            />
            <InfoComp
                title="Version"
                value="v1.1"
            />
            <InfoComp
                title="License"
                value="Created for Alperia AG"
            />
        </>
    )
}