import { useEffect, useState } from "react";
import { Socket } from "phoenix";

export default function CountingPage() {
  const [count, setCount] = useState(0);
  const [channel, setChannel] = useState<any>(null);

  useEffect(() => {
    const protocol = window.location.protocol === "https:" ? "wss" : "ws";
    const socket = new Socket(`${protocol}://${window.location.host}/ws/socket`);
    socket.connect();

    const chan = socket.channel("counter:lobby", {});
    chan.join()
      .receive("ok", () => console.log("Connected to counter channel"))
      .receive("error", () => console.log("Failed to join"));

    chan.on("count_update", (payload: any) => setCount(payload.count));

    setChannel(chan);

    return () => {
      chan.leave();
      socket.disconnect();
    };
  }, []);

  const increment = () => {
    if (channel) channel.push("increment", {});
  };

  return (
    <div style={{ textAlign: "center", marginTop: "2rem" }}>
      <h1>Shared Counter</h1>
      <p style={{ fontSize: "2rem" }}>{count}</p>
      <button onClick={increment} style={{ padding: "1rem 2rem", fontSize: "1.5rem" }}>
        Increment
      </button>
    </div>
  );
}
