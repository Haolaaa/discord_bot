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
        cmd!(general, pfp),
        cmd!(general, age),
        cmd!(music, skip),
        cmd!(music, play),
        cmd!(voice, join),
        cmd!(voice, leave),
    ]
}
