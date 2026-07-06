import Link from 'next/link';
import style from '@styles/layouts/navbarDesktop.module.scss';
import { Field, Label } from '@components/ui/chormaticUI';
import { IoSettingsOutline } from "react-icons/io5";
import { IoHomeOutline } from "react-icons/io5";
import { IoSearchOutline } from "react-icons/io5";
import { IoIosNotificationsOutline } from "react-icons/io";
import { IoBookmarkOutline } from "react-icons/io5";
import { IoChatbubbleOutline } from "react-icons/io5";
import { HiOutlinePlus } from "react-icons/hi";

const NavDesktop: React.FC = () => {

    // * create array for nav then for-loop
    return (
    <nav className={style.navbar}>
        <div className={style.navTop}>
            <Link href="/">
                <img src="/icons/logo.png" className={style.navLogo}/>
            </Link>
        </div>
        <Field className={style.navMid}>
            <ul style={{width: "100%"}}>
                <li>
                    <Link className={style.flex} href={'#'}>
                        <IoHomeOutline/><p className={style.hidden}>Home</p>
                    </Link>
                </li>
                
                <li>
                    <Link className={style.flex} href={'#'}> 
                        <IoSearchOutline/><p className={style.hidden}>Search</p>
                    </Link>
                </li>
                
                <li>
                    <Link className={style.flex} href={'#'}> 
                        <IoIosNotificationsOutline/><p className={style.hidden}>Notification</p>
                    </Link>
                </li>
                
                <li>
                    <Link className={style.flex} href={'#'}> 
                        <IoBookmarkOutline/><p className={style.hidden}>Bookmark</p>
                    </Link>
                </li>
                
                <li>
                    <Link className={style.flex} href={'#'}> 
                        <IoChatbubbleOutline/><p className={style.hidden}>Messages</p>
                    </Link>
                </li>

                <li>
                    <Link className={`${style.flex} ${style.post}`} href={'#'}> 
                        <HiOutlinePlus/><p className={style.hidden}>Post</p>
                    </Link>
                </li>
            </ul>
        </Field>
        
        <Field className={style.navBottom}>
            <Field orientation={'horizontal'}>
                <div style={{display: 'flex', gap: '4px'}}>
                    <img src="https://placehold.co/40" alt="" />
                    <Label className={style.hidden}>Username</Label>
                </div>
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