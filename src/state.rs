#[derive(Default)]
pub struct VRChatState {
    pub instance_users: Vec<String>,
    pub instance_owner: String,
    pub instance_permission: String,
}
