import { AdminGuard } from "./adminGuard";

export default function AdminLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return <AdminGuard>{children}</AdminGuard>;
}
