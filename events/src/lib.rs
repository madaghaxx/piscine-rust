use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.position,
            self.size,
            self.content.truecolor(self.color.0, self.color.1, self.color.2)
        )
    }
}

use Event::*;

impl Event<'_> {
    pub fn notify(&self) -> Notification {
        match self {
            Event::Remainder(text) =>
                Notification {
                    size: 50,
                    color: (50, 50, 50),
                    position: Position::Bottom,
                    content: format!("{}", text),
                },
            Event::Registration(d) => {
				let (hours, minutes, seconds) = (d.num_hours(), d.num_minutes() % 60, d.num_seconds() % 60);
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {}H:{}M:{}S left before the registration ends",
                        hours,
                        minutes,
                        seconds
                    ),
                }
            }
            Event::Appointment(text) =>
                Notification {
                    size: 100,
                    color: (200, 200, 3),
                    position: Position::Center,
                    content: format!("{}", text),
                },
            _ =>
                Notification {
                    size: 25,
                    color: (0, 255, 0),
                    position: Position::Top,
                    content: format!("Enjoy your holiday"),
                },
        }
    }
}
