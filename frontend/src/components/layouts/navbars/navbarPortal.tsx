import React from 'react';
import Link from 'next/link';
import style from '@styles/layouts/navbarDesktop.module.scss';

const NavPortal: React.FC = () => {
    return (
    <nav className={style.navbar}>
        <div className={style.navTopPortal}>
            <Link href="/">
                {/* <img src="/icons/logo.png" className={style.navLogo}/> */}
            </Link>
            {/* Profile */}
        </div>
    </nav>
    )
}

export default NavPortal;