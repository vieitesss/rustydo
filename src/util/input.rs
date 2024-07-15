pub struct Input {
    text: String,
    text_bounds: (u16, u16),
    text_pos: u16,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            text: String::new(),
            text_bounds: (0, 0),
            text_pos: 0,
        }
    }
}

impl Input {
    pub fn new() -> Input {
        Self::default()
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }

    pub fn get_text_pos(&self) -> u16 {
        self.text_pos
    }

    pub fn increase_text_pos(&mut self) {
        self.text_pos += 1;
    }

    pub fn decrease_text_pos(&mut self) {
        self.text_pos -= 1;
    }

    pub fn set_text_bounds(&mut self, left: u16, right: u16) {
        self.text_bounds = (left, right);
    }

    pub fn get_text_bounds(&mut self) -> (u16, u16) {
        self.text_bounds
    }

    pub fn get_text_max_len(&mut self) -> u16 {
        self.text_bounds.1 - self.text_bounds.0
    }

    pub fn clear(&mut self) {
        self.text = String::new();
        self.text_pos = 0;
    }

    pub fn insert_char(&mut self, c: char) {
        self.text += &c.to_string();
        self.text_pos += 1;
    }

    pub fn remove_char(&mut self) {
        let text_len = self.text.len();

        if self.text_pos > 0 {
            self.decrease_text_pos();
        }

        let chars = self.text.chars();
        let lhs: String = chars.clone().take(self.text_pos as usize).collect();
        let rhs: String = chars
            .clone()
            .skip(self.text_pos as usize + 1)
            .take(text_len - self.text_pos as usize)
            .collect();

        self.text = String::from(format!("{}{}", lhs, rhs));
    }
}
