use crate::domain::user::types::DisplayName;

pub struct UserProfile {
    pub user_id: i64,
    pub display_name: Option<DisplayName>,
    pub bio: Option<String>, // ! unset
    pub avatar_url: Option<String>, // ! unset
}

impl UserProfile {
    pub fn set_display_name(&mut self, display_name: Option<DisplayName>) {
        self.display_name = display_name;
    }
}