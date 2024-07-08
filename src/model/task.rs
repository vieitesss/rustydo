pub struct Task {
    // TODO: add uuid
    // TODO: add area
    title: String,
    done: bool,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Task {
            title: title.to_string(),
            done: false,
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
}
