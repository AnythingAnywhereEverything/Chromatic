import { useRouter } from "next/router";

export default function ChatPage() {
  const router = useRouter();
  const { id } = router.query;

  return (
    <div>
        <h1>Chat Room #{id}</h1>
        <p>Messages will appear here.</p>
    </div>
  );
}
