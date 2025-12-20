use tome_cabi_types::TomeChatRole;
use tome_core::Rope;

#[derive(Debug)]
pub struct ChatPanelState {
	pub title: String,
	pub input: Rope,
	pub input_cursor: usize,
	pub transcript: Vec<ChatItem>,
}

#[derive(Debug)]
pub struct ChatItem {
	pub _role: TomeChatRole,
	pub _text: String,
}

impl ChatPanelState {
	pub fn new(title: String) -> Self {
		Self {
			title,
			input: Rope::from(""),
			input_cursor: 0,
			transcript: Vec::new(),
		}
	}
}
