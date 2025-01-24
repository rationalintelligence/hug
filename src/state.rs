use ratatui::Frame;

pub struct AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        let text = String::from("hug");
        frame.render_widget(text, frame.area());
    }
}
