"use client";

import React from "react";
import { ping } from "../../api/ping";
import style from "@styles/home.module.scss";

const Home: React.FC = () => {
    const [message, setMessage] = React.useState("Pinging...");

    // const { data: user, isLoading } = useUser();
    // const router = useRouter();

    // Commented  for frontend only development purposes, to allow access to authenticated pages without login
    // useEffect(() => {
    //     if (!isLoading && !user) {
    //         router.replace("/");
    //     }
    // }, [user, isLoading, router]);

    // if (isLoading || !user) {
    //     return (
    //         <div>
    //             <p>Loading application...</p>
    //         </div>
    //     );
    // }

    React.useEffect(() => {
        async function fetchPing() {
            try {
                const res = await ping();
                setMessage(res);
            } catch (err) {
                console.error(err);
                setMessage("API failed");
            }
        }
        fetchPing();
    }, []);

    return (
        <main>
            <div className={style.homeContainer}>
                <header>
                    <h1>Welcome to Next.js Absolute Cinema Testing!</h1>
                    <p>Ping response: {message}</p>
                </header>

                <main>
                    <p>This is a template homepage example</p>
                </main>
                <footer>
                    <small>Footer!</small>
                </footer>
            </div>
        </main>
    );
};

export default Home;
