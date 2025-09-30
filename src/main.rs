mod emulator;
mod ui;

use iced::{
    widget::{column, Button, Container, Row, Text},
    Alignment, Element,
};

use iced_aw::ContextMenu;

use iced::{Application, Settings};
use ui::app::RGenApp;

fn main() -> iced::Result {
    RGenApp::run(Settings {
        antialiasing: true,
        default_font: None,
        window: iced::window::Settings {
            size: (800, 600),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    });
    iced::application(
        ContextMenuRGen::default,
        ContextMenuRGen::update,
        ContextMenuRGen::view,
    )
    .run()
}

#[derive(Clone, Debug)]
pub enum Message {
    ButtonClicked,
    File,
    Emulation,
    Video,
    Audio,
    Input,
    Tools,
    Options,
    Help,
}

#[derive(Default)]
struct ContextMenuRGen {
    last_message: Option<Message>,
}

impl ContextMenuRGen {
    fn update(&mut self, message: Message) {
        self.last_message = Some(message);
    }

    fn view(&self) -> Element<'_, Message> {
        let underlay = Container::new(
            Row::new()
                .spacing(10)
                .align_y(Alignment::Center)
                .push(Button::new(Text::new("right click me!")).on_press(Message::ButtonClicked))
                .push(Text::new(format!(
                    "Last message: {}",
                    match self.last_message.as_ref() {
                        Some(message) => match message {
                            Message::ButtonClicked => "button clicked",
                            Message::File => "file",
                            Message::Emulation => "emulation",
                            Message::Video => "video",
                            Message::Audio => "audio",
                            Message::Input => "input",
                            Message::Tools => "tools",
                            Message::Options => "options",
                            Message::Help => "help",
                        },
                        None => "None",
                    }
                ))),
        );

        ContextMenu::new(underlay, || {
            column(vec![
                iced::widget::button("File")
                    .on_press(Message::File)
                    .into(),
                iced::widget::button("Emulation")
                    .on_press(Message::Emulation)
                    .into(),
                iced::widget::button("Video")
                    .on_press(Message::Video)
                    .into(),
                iced::widget::button("Audio")
                    .on_press(Message::Audio)
                    .into(),
                iced::widget::button("Input")
                    .on_press(Message::Input)
                    .into(),
                iced::widget::button("Tools")
                    .on_press(Message::Tools)
                    .into(),
                iced::widget::button("Options")
                    .on_press(Message::Options)
                    .into(),
                iced::widget::button("Help")
                    .on_press(Message::Help)
                    .into(),
            ])
            .into()
        })
        .into()
    }
}
