import React from 'react';
import Link from 'next/link';
import style from '@styles/layouts/navbarDesktop.module.scss';

const NavAuth: React.FC = () => {
    return (
    <nav className={style.navbar}>
        <div className={style.navTop}>
            <Link href="/">
                {/* <img src="#" className={style.navLogo}/> */}
            </Link>
            <div></div>
        </div>
    </nav>
    )
}

export default NavAuth;