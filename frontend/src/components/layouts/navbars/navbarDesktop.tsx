import Link from 'next/link';
import style from '@styles/layouts/navbarDesktop.module.scss';
import { Field, Label } from '@components/ui/chormaticUI';
import { IoSettingsOutline } from "react-icons/io5";
import { IoHomeOutline } from "react-icons/io5";
import { IoSearchOutline } from "react-icons/io5";
const NavDesktop: React.FC = () => {

    return (
    <nav className={style.navbar}>
        <div className={style.navTop}>
            <Link href="/">
                <img src="/icons/logo.png" className={style.navLogo}/>
            </Link>
        </div>
        <Field className={style.navMid}>
            <ul>
                <li><IoHomeOutline/> <Link className={style.hidden} href={'#'}>Home</Link></li>
                <li><IoSearchOutline/><Link className={style.hidden} href={'#'}>Search</Link></li>
                <li><Link className={style.hidden} href={'#'}>Notification</Link></li>
                <li><Link className={style.hidden} href={'#'}>Bookmark</Link></li>
                <li><Link className={style.hidden} href={'#'}>Chat</Link></li>
                <li><Link className={style.hidden} href={'#'}>Post</Link></li>
            </ul>
        </Field>
        <Field className={style.navBottom}>
            <Field orientation={'horizontal'}>
                <div style={{display: 'flex', gap: '4px'}}>
                    <img src="https://placehold.co/60" alt="" />
                    <Label className={style.hidden}>Username</Label>
                </div>

                <IoSettingsOutline/>
            </Field>
        </Field>
    </nav>
    )
}

const NavFriendList: React.FC = () => {
    return (
        <>

        </>      
    );
};
export default NavDesktop;