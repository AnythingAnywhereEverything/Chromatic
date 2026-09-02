"use client";

import { FaBell, FaMagnifyingGlass } from "react-icons/fa6";
import style from "./style.module.scss";
import Link from "next/link";

export default function Topbar() {
    return (
        <div className={style["top-bar"]}>
            <div className={style["top-bar-logo"]}>
                <img src="/asset/logo_no_border.svg" alt="Chromatic Logo" />
                <h1>Chromatic</h1>
            </div>
            <div className={style["items"]}>
                <div>
                    <FaMagnifyingGlass /> 
                </div>
                <Link href="/notifications" className={style["item"]}>
                    <FaBell />
                </Link>
            </div>
        </div>
    );
}
