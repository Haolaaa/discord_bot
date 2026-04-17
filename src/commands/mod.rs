pub mod general;
pub mod music;
pub mod voice;

use crate::{client::BotData, error::BotError};

type Command = poise::Command<BotData, BotError>;

#[macro_export]
macro_rules! cmd {
    ($module: ident, $name: ident) => {
        $module::$name::$name()
    };

    ($module: ident, $name: ident, $func: ident) => {
        $module::$name::$func()
    };
}

pub fn all() -> Vec<Command> {
    vec![
        // general
        cmd!(general, pfp),
        cmd!(general, age),
        // music
        cmd!(music, skip),
        cmd!(music, play),
        cmd!(music, pause),
        cmd!(music, resume),
        cmd!(music, stop),
        // voice
        cmd!(voice, join),
        cmd!(voice, leave),
    ]
}
