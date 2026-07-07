"use client";

import { FaBell } from "react-icons/fa6";
import style from "./style.module.scss";
import Link from "next/link";

export default function Topbar() {
    return (
        <div className={style["top-bar"]}>
            <div className={style["top-bar-logo"]}>
                <h1>Chromatic</h1>
            </div>
            <div className={style["items"]}>
                <input type="text" placeholder="Search..." className={style["search"]} />
                <Link href="/notifications" className={style["item"]}><FaBell /></Link>
            </div>
        </div>
    );
}