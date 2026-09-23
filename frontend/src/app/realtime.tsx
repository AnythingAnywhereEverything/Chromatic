"use client";

import {
    createContext,
    useContext,
    useEffect,
    useMemo,
    useRef,
    useState,
    type ReactNode,
} from "react";
import { Channel, Socket } from "phoenix";
import { useUser } from "@/hooks/useUser";
import { getToken } from "@/handler/token_handler";

type Message = {
    id: string;
    sender_id: string;
    recipient_id: string;
    content: string;
};

type RealtimeContextValue = {
    connected: boolean;
    message: Message | null;
    sendMessage: (recipientId: string, content: string) => void;
};

const RealtimeContext = createContext<RealtimeContextValue | null>(null);

export function RealtimeProvider({
    children,
}: {
    children: ReactNode;
}) {
    const userId = useUser().data?.id;

    const [token, setToken] = useState<string | null>(null);
    const [connected, setConnected] = useState(false);
    const [message, setMessages] = useState<Message|null>(null);

    const channelRef = useRef<Channel | null>(null);

    useEffect(() => {
        setToken(getToken());
    }, []);

    const socket = useMemo(() => {
        if (!token) {
            return null;
        }

        return new Socket(process.env.NEXT_PUBLIC_PHOENIX_SOCKET_URL || "ws://localhost:4000/socket", {
            params: {
                token,
            },
        });
    }, [token]);

    useEffect(() => {
        if (!socket || !userId) {
            return;
        }

        socket.connect();

        const channel = socket.channel(`dm:${userId}`, {});

        channelRef.current = channel;

        channel
            .join()
            .receive("ok", () => {
                console.log("DM channel connected");
                setConnected(true);
            })
            .receive("error", (error) => {
                console.error("DM channel failed", error);
            });

        channel.on("new_message", (message: Message) => {
            setMessages(message);
        });

        return () => {
            channel.leave();
            channelRef.current = null;

            socket.disconnect();
            setConnected(false);
        };
    }, [socket, userId]);

    const sendMessage = (
        recipientId: string,
        content: string,
    ) => {
        channelRef.current?.push("send_message", {
            recipient_id: recipientId,
            content,
        });
    };

    return (
        <RealtimeContext.Provider
            value={{
                connected,
                message,
                sendMessage,
            }}
        >
            {children}
        </RealtimeContext.Provider>
    );
}

export function useRealtime() {
    const context = useContext(RealtimeContext);

    if (!context) {
        throw new Error(
            "useRealtime must be used inside RealtimeProvider",
        );
    }

    return context;
}