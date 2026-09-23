import MessageContainer from "./body";

export const metadata = {
    title: "Chromatic - Messages",
};

export default async function MessagePage() {
    // Strictly import and use the components hete
    // due to it being a SSR page, and not a client component. This is to avoid hydration errors.
    // ? Loading friends/messages would go here

    return (
        <MessageContainer />
    );
}
