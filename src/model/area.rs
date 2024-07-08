pub struct Area {
    title: String,
}

impl Area {
    pub fn new(title: String) -> Self {
        Area {
            title
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}
