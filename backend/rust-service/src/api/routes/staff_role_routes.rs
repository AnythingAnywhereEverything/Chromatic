use axum::{Router, routing::get};

use crate::{
    api::handlers::staff_role_handlers::{
        create_role_handler, delete_role_handler, get_role_handler, get_user_staff_roles_handler,
        list_role_members_handler, list_roles_handler, set_user_staff_roles_handler,
        update_role_handler,
    },
    application::state::SharedState,
};

/// Staff role management, merged into the existing `/admin` router.
///
/// `merge`, not `nest`: axum rejects nesting at the root path with "Nesting at
/// the root is no longer supported". Since these routes are meant to sit at the
/// same level as `/admin/users`, merging their absolute paths is exactly right.
///
/// Nested rather than given its own top-level nest because these routes are the
/// admin panel's role editor and nowhere else — they are gated by exactly the
/// same `AdminUser` extractor as every other admin route, so there is no
/// separate surface to reason about and no second place where the superuser check
/// has to be remembered.
///
/// `PUT` on the user's role set rather than `POST .../roles/{id}` and
/// `DELETE .../roles/{id}`: the body is the complete desired set, which lets the
/// server diff it and audit each grant and revoke individually. Two endpoints
/// would mean two round trips and no reliable diff.
///
/// `DELETE /roles/{role_id}` is the hard delete the schema forces — there is no
/// `deleted_at` on `staff_roles` — and the join table cascades with it.
pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/roles", get(list_roles_handler).post(create_role_handler))
        .route(
            "/roles/{role_id}",
            get(get_role_handler)
                .patch(update_role_handler)
                .delete(delete_role_handler),
        )
        .route("/roles/{role_id}/members", get(list_role_members_handler))
        .route(
            "/users/{user_id}/staff-roles",
            get(get_user_staff_roles_handler).put(set_user_staff_roles_handler),
        )
}
