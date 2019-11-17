pub struct TabsState {
    pub index: usize,
}

impl TabsState {
    pub const TITLES: &'static [&'static str] = &["Subscriptions", "Stream", "Retain"];

    pub fn default() -> TabsState {
        TabsState { index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % Self::TITLES.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = Self::TITLES.len() - 1;
        }
    }
}
