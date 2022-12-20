import Head from "next/head"

export default function Loading() {
    return (
        <>
        <Head>
            <title>Loading...</title>
        </Head>
        <div className="flex flex-col items-center justify-center h-full">
            <img src="/images/loading_animation.gif" alt="Loading" className="max-w-full h-auto" />
            <h1 className="text-2xl font-bold text-center">Loading... Please wait</h1>
        </div>
        </>
    )
}