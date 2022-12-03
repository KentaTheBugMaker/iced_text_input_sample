use iced::executor;

use iced::widget::row;
use iced::widget::Text;
use iced::widget::TextInput;
use iced::widget::{container, Column};
use iced::{Application, Command, Length, Settings, Theme};

fn main() -> Result<(), iced::Error> {
    TextInputSample::run(Settings {
        default_font: Some(include_bytes!("../font/NotoSansCJKjp-Regular.otf")),
        exit_on_close_request: true,
        ..Default::default()
    })
}

struct TextInputSample {
    text_buffers: [String; 2],
    password_buffer: String,
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    xdg_session_type: String,
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    ime_name: String,
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    winit_backend: String,
}
#[derive(Debug, Clone)]
enum Message {
    Input1Changed(String),
    Input2Changed(String),
    PasswordChanged(String),
}
impl iced::Application for TextInputSample {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                text_buffers: [String::new(), String::new()],

                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                xdg_session_type: std::env::var("XDG_SESSION_TYPE").unwrap_or("UNKNOWN".to_owned()),
                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                ime_name: std::env::var("GTK_IM_MODULE")
                    .or_else(|_| std::env::var("QT_IM_MODULE"))
                    .or_else(|_| std::env::var("XMODIFIERS"))
                    .unwrap_or("UNKNOWN".to_owned()),
                #[cfg(not(any(target_os = "windows", target_os = "macos")))]
                winit_backend: std::env::var("WINIT_UNIX_BACKEND").unwrap_or("Not Set".to_owned()),
                password_buffer: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "IME debugger".to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Input1Changed(text) => self.text_buffers[0] = text,
            Message::Input2Changed(text) => self.text_buffers[1] = text,

            Message::PasswordChanged(text) => self.password_buffer = text,
        }
        Command::none()
    }
    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = Column::new()
            .push(row(vec![
                Text::new("TextInput 1").into(),
                TextInput::new(
                    "please input some text here",
                    &self.text_buffers[0],
                    Message::Input1Changed,
                )
                .into(),
            ]))
            .push(row(vec![
                Text::new("TextInput 2").into(),
                TextInput::new(
                    "please input some text here",
                    &self.text_buffers[1],
                    Message::Input2Changed,
                )
                .into(),
            ]))
            .push(row(vec![
                Text::new("PassWord Input ").into(),
                TextInput::new(
                    "please input some text here",
                    &self.password_buffer,
                    Message::PasswordChanged,
                )
                .password()
                .into(),
                Text::new(&self.password_buffer).into(),
            ]));

        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let ime_info = Column::with_children(vec![
            Text::new(format!("Input Method FrameWork {}", self.ime_name)).into(),
            Text::new(format!("Display server {}", self.xdg_session_type)).into(),
            Text::new(format!("Winit backend {}", self.winit_backend)).into(),
        ]);
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        let content = content.push(ime_info.into());

        container(content)
            .height(Length::Shrink)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
