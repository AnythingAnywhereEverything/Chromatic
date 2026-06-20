import React from 'react';
import NavDesktop from './navbars/navbarDesktop';
import NavAuth from './navbars/navbarLogin';
import NavPortal from './navbars/navbarPortal';

interface NavProps {
  type?: 'auth' | 'portal';
}

const Navbar: React.FC<NavProps> = ({type}) => {
    if (type === "auth"){
        return (
            <NavAuth/>
        )
    } else if (type === "portal")
        return <NavPortal/>
    return (
        <NavDesktop/>
    );
}

export { Navbar };