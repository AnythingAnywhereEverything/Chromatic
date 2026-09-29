import { redirect } from "next/navigation";

// ? Should I redirect to the overview page?
export default function AdminPage() {
    redirect("/admin/overview");
}