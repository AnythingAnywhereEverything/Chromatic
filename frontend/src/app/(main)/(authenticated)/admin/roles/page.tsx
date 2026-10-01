import AdminRolesBody from "./body";

/**
 * Staff roles list.
 *
 * A Server Component on purpose: it fetches nothing and only renders the client
 * body. Admin data cannot be loaded here because the auth token lives in
 * localStorage, which is not available during server render.
 */
export default function AdminRolesPage() {
    return <AdminRolesBody />;
}