//! This example shows how to use touch events in `Canvas` to draw
//! a circle around each fingertip. This only works on touch-enabled
//! computers like Microsoft Surface.
use iced::{keyboard, mouse, Size};
use iced::widget::canvas::{event, LineCap, LineJoin};
use iced::widget::canvas::stroke::{self, Stroke};
use iced::widget::canvas::{self, Canvas, Geometry};
use iced::{
    executor, touch, window, Application, Color, Command, Element, Length,
    Point, Rectangle, Renderer, Settings, Subscription, Theme,
};

use std::collections::HashMap;
use iced::application::{Appearance, StyleSheet};
use iced::mouse::Event;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    Painter::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            transparent: true,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Painter {
    state: State,
}

#[derive(Debug)]
struct State {
    cache: canvas::Cache,
    positions: Vec<Point>,
    drawing: bool,
}

impl State {
    fn new() -> Self {
        Self {
            cache: canvas::Cache::new(),
            positions: Vec::new(),
            drawing: false,
        }
    }
}

#[derive(Debug)]
enum Message {
    LeftButtonDown { position: Point },
    LeftButtonUp {},
    MouseDragged { position: Point },
    Reset {},
    Exit {},
}

struct TransparentStyle {

}

impl StyleSheet for TransparentStyle {
    type Style = ();

    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance {
            background_color: Color::TRANSPARENT,
            text_color: Color::BLACK
        }
    }
}

impl Application for Painter {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Painter {
                state: State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("VivoPaint - Iced")
    }

    fn theme(&self) -> Theme {
        Theme::custom(iced::theme::Palette {
            background: Color::TRANSPARENT,
            // background: Color::from_rgb(1.0, 0.0, 0.0),
            text: Color::BLACK,
            primary: Color::from_rgb(0.5, 0.5, 0.0),
            success: Color::from_rgb(0.0, 1.0, 0.0),
            danger: Color::from_rgb(1.0, 0.0, 0.0),
        })
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LeftButtonDown { position } => {
                println!("Left button pressed at: {}, {}", position.x, position.y);
                self.state.positions.push(position);
                self.state.cache.clear();
                self.state.drawing = true;
            }
            Message::MouseDragged { position } => {
                if self.state.drawing {
                    self.state.positions.push(position);
                    self.state.cache.clear();
                    println!("state.positions.size: {}", self.state.positions.len());
                }
            }
            Message::LeftButtonUp { .. } => {
                println!("Left button lifted");
                self.state.drawing = false;
            }
            Message::Reset { .. } => {
                self.state.positions.clear();
                self.state.cache.clear();
            }
            Message::Exit { .. } => {
                std::process::exit(0);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }

    fn view(&self) -> Element<Message> {
        Canvas::new(&self.state)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl canvas::Program<Message, Renderer> for State {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: event::Event,
        _bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> (event::Status, Option<Message>) {

        match event {
            event::Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let position = cursor.position().unwrap();
                    (
                        event::Status::Captured,
                        Some(Message::LeftButtonDown { position }),
                    )
                }
                mouse::Event::CursorMoved { position } => {
                    (
                        event::Status::Captured,
                        Some(Message::MouseDragged { position }),
                    )
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    (
                        event::Status::Captured,
                        Some(Message::LeftButtonUp {}),
                    )
                }
                _ => (event::Status::Ignored, None),
            }
            event::Event::Keyboard(keyboard_event) => match keyboard_event {
                keyboard::Event::KeyPressed { key_code, .. } => match key_code {
                    keyboard::KeyCode::Escape => {
                        (
                            event::Status::Captured,
                            Some(Message::Exit {}),
                        )
                    }
                    keyboard::KeyCode::R => {
                        (
                            event::Status::Captured,
                            Some(Message::Reset {}),
                        )
                    }
                    _ => (event::Status::Ignored, None),
                },
                _ => (event::Status::Ignored, None),
            }
            ,
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {

        let path_shape = self.cache.draw(renderer, bounds.size(), |frame| {

            if self.positions.len() < 2 {
                return;
            }

            let mut builder = canvas::path::Builder::new();

            for (index, p) in self.positions.iter().enumerate() {
                let p = Point::new(p.x, p.y);

                match index {
                    0 => builder.move_to(p),
                    _ => builder.line_to(p),
                }
            }

            let path = builder.build();

            frame.stroke(
                &path,
                Stroke {
                    style: stroke::Style::Solid(Color::from_rgba(1.0, 0.0, 0.0, 0.5)),
                    line_cap: LineCap::Round,
                    line_join: LineJoin::Round,
                    width: 10.0,
                    ..Stroke::default()
                },
            );
        });

        vec![path_shape]
    }
}
