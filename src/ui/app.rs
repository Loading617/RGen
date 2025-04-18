use iced::{executor, Application, Command, Element, Theme, widget::{Button, Column, Text}};
use crate::emulator::RGen;

pub struct RGenApp {
    emulator: RGen,
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadRom,
    Tick,
}

impl Application for RGenApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        (
            Self {
                emulator: RGen::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("RGen")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LoadRom => {
            }
            Message::Tick => {
                self.emulator.tick_frame();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        Column::new()
            .push(Button::new("Load ROM").on_press(Message::LoadRom))
            .push(Text::new("RGen UI"))
            .into()
    }
}
