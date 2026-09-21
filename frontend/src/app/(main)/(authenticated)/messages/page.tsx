import { getPublicUserProfile } from "@/api/user/profile";
import { MessageList } from "@/app/_components/ui/chromatic/message/messageList";
import { cache } from "react";
import style from "./message.module.scss";

export const metadata = {
    title: "Chromatic - Messages",
};

export default async function MessagePage({}: {}) {
    // Strictly import and use the components hete
    // due to it being a SSR page, and not a client component. This is to avoid hydration errors.
    // ? Loading friends/messages would go here

    return (
        <div className={style["message-layout"]}>
            <MessageList />
            <div className={style["message-body"]}>
                {/* Import and use the MessageBody component here */}
                Giviing some message1
            </div>
        </div>
    );
}
