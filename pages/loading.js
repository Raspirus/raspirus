import Head from "next/head";
import styles from '../styles/animation.module.css';

export default function Loading() {
    return (
        <>
        <Head>
            <title>Loading...</title>
        </Head>
        <main className="flex flex-col items-center justify-center h-full">
            <h1 className="text-2xl font-bold text-center">Loading... Please wait</h1>
            <div className="flex flex-row m-10">
                <div className={[styles.main_div, styles.zero_div].join(" ")}></div>
                <div className={[styles.main_div, styles.first_div].join(" ")}></div>
                <div className={[styles.main_div, styles.second_div].join(" ")}></div>
                <div className={[styles.main_div, styles.third_div].join(" ")}></div>
                <div className={[styles.main_div, styles.fourth_div].join(" ")}></div>
            </div>
        </main>
        </>
    )
}