"use client";

import React from "react";
import { ping } from "../../api/ping";
import style from "@styles/home.module.scss";
import { useLayer } from "@/app/_components/layer";
// import { Tooltip } from "@/app/_components/ui/ztx-tooltips";
// import { useRouter } from "next/navigation";
// import { useUser } from "@/hooks/useUser";

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

    const layer = useLayer();

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

    const exampleOpenLayer = () => {
        if (!layer) {
            console.error("Layer context is not available.");
            return;
        }
        const overlay = layer.open(
            <div className={style.overlay}>
                <h2>Overlay Content</h2>
                <p>This is an example overlay.</p>
                <button onClick={() => overlay.close()}>Close Overlay</button>
            </div>,
        );
    };

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
