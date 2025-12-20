use tome_cabi_types::TomeChatRole;
use tome_core::Rope;

#[derive(Debug)]
pub struct ChatPanelState {
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
	pub fn new(_title: String) -> Self {
		Self {
			input: Rope::from(""),
			input_cursor: 0,
			transcript: Vec::new(),
		}
	}
}
