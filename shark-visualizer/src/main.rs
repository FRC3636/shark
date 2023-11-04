use iced::{
    executor,
    widget::{button, row},
    Application, Command, Settings, Theme,
};

fn main() {
    Visualizer::run(Settings::default()).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Play,
    Pause,
    Step,
}

struct Visualizer {}
impl Application for Visualizer {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self {}, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("🦈 Shark Visualizer 🦈")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        row![
            button("Play").on_press(Message::Play),
            button("Pause").on_press(Message::Pause),
            button("Step").on_press(Message::Step),
        ].into()
    }
}
