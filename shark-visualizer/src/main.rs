use clap::Parser;
use iced::{
    executor, mouse,
    widget::{
        button, canvas,
        canvas::{Cache, Geometry, Path, Program},
        column, row,
    },
    Application, Color, Command, Length, Point, Rectangle, Renderer, Settings, Size, Theme,
};
use palette::{FromColor, IntoColor, Srgb};
use shark::{primitives::*, shader::*};

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 20)]
    num_pixels: usize,

    #[arg(short, long, default_value_t = 20.0)]
    tps: f64,
}

struct VisualizerSettings<S: Shader<FragOne>> {
    args: Args,
    shader: S,
}

fn main() {
    let shader = checkerboard(position_gradient(random(), off(), |i| i as f32 / 20.0), color(Srgb::new(1.0, 0.9, 0.5)), 2);
    let settings = VisualizerSettings {
        args: Args::parse(),
        shader,
    };
    Visualizer::run(Settings::with_flags(settings)).unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Message {
    Play,
    Pause,
    Step,
    Tick,
}

struct Visualizer<S: Shader<FragOne>> {
    paused: bool,
    visualization: Visualization,
    tps: f64,
    shader: S,
}
impl<S: Shader<FragOne>> Application for Visualizer<S> {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = VisualizerSettings<S>;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                paused: true,
                shader: flags.shader,
                tps: flags.args.tps,
                visualization: Visualization {
                    cache: Cache::default(),
                    colors: vec![Srgb::new(0.0, 0.0, 0.0); flags.args.num_pixels],
                },
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("🦈 Shark Visualizer 🦈")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Play => self.paused = false,
            Message::Pause => self.paused = true,
            Message::Step => {
                for (index, color) in self.visualization.colors.iter_mut().enumerate() {
                    let new_color = self
                        .shader
                        .shade(FragOne {
                            pos: index,
                            time: 0.0,
                        })
                        .into_color();
                    *color = Srgb::from_color(new_color);
                }
                self.visualization.cache.clear();
            }
            Message::Tick => {
                if !self.paused {
                    for (index, color) in self.visualization.colors.iter_mut().enumerate() {
                        let new_color = self
                            .shader
                            .shade(FragOne {
                                pos: index,
                                time: 0.0,
                            })
                            .into_color();
                        *color = Srgb::from_color(new_color);
                    }
                    self.visualization.cache.clear();
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        column![
            row![
                button("Play").on_press(Message::Play),
                button("Pause").on_press(Message::Pause),
                button("Step").on_press(Message::Step),
            ],
            canvas(&self.visualization).width(Length::Fill).height(200)
        ]
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::time::every(std::time::Duration::from_secs_f64(1.0 / self.tps)).map(|_| Message::Tick)
    }
}

#[derive(Debug)]
struct Visualization {
    cache: Cache,
    colors: Vec<Srgb>,
}

impl Program<Message, Renderer> for Visualization {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let visualization = self.cache.draw(renderer, bounds.size(), |frame| {
            let width = frame.width() / self.colors.len() as f32;
            let height = frame.height();
            for (index, color) in self.colors.iter().enumerate() {
                let rect = Path::rectangle(
                    Point::new(width * index as f32, 0.0),
                    Size::new(width, height),
                );

                frame.fill(
                    &rect,
                    Color {
                        r: color.red,
                        g: color.green,
                        b: color.blue,
                        a: 1.0,
                    },
                )
            }
        });
        vec![visualization]
    }
}
