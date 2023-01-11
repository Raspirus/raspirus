import Head from "next/head"
import { useRouter } from "next/router";
import InfoComp from "../components/info-comp";
import { faR, faBook, faUser, faInfoCircle, faScaleBalanced } from '@fortawesome/free-solid-svg-icons';

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
                <button onClick={backHome} type="button" className="inline-block align-middle px-6 py-2.5 m-2 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out">
                    <i className="pr-1 fa fa-home"></i>
                    Home
                </button>
                <h1 className="inline-block align-middle p-2 font-medium leading-tight text-5xl mt-0 mb-2 text-mainred">Information</h1>
            </div>
            <InfoComp
                title="App Name"
                value="Raspirus"
                icon={faR}
            />
            <InfoComp
                title="Description"
                value="Simple signatures-based antivirus for single-board computers like Raspbrry Pi"
                icon={faBook}
            />
            <InfoComp
                title="Contributors"
                value="Demetz Benjamin, Hell Björn Felix"
                icon={faUser}
            />
            <InfoComp
                title="Version"
                value="v1.1"
                icon={faInfoCircle}
            />
            <InfoComp
                title="License"
                value="Private"
                icon={faScaleBalanced}
            />
        </>
    )
}