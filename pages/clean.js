import Head from "next/head";
import { useRouter } from 'next/router';
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faHome } from "@fortawesome/free-solid-svg-icons";

export default function Clean() {
    const router = useRouter();

    const backHome = () => {
        router.push('/');
    }

    return (
        <>
            <Head>
                <title>No Virus found</title>
            </Head>
            <div className="flex items-center justify-center h-screen flex-col">
                <h1 className="text-center mb-10 pt-4 font-medium leading-tight text-5xl mt-0 text-maingreen">No virus found</h1>
                <img src="/images/success_image.png" alt="Success" className="max-w-[30%]" />
                <button onClick={backHome} type="button" className="inline-block px-6 py-2.5 m-10 bg-mainred text-white font-medium text-xs leading-tight uppercase rounded shadow-md hover:bg-mainred-dark hover:shadow-lg focus:bg-mainred-dark focus:shadow-lg focus:outline-none focus:ring-0 active:bg-mainred-dark active:shadow-lg transition duration-150 ease-in-out">
                    <FontAwesomeIcon
                        icon={faHome}
                        size="1x"
                        className="pr-1"
                    />
                    Home
                </button>
            </div>
        </>
    )
}