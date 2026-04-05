// src/api/ping.tsx
export async function ping(): Promise<string> {
  const res = await fetch("/api/v2/health"); // proxied to backend via nginx
  if (!res.ok) throw new Error("Ping failed");
  const data = await res.json();
  return data.status;
}
