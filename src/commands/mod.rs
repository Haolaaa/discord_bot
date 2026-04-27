pub mod general;
pub mod music;
pub mod queue;
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
        cmd!(music, play),
        cmd!(music, pause),
        cmd!(music, resume),
        cmd!(music, stop),
        cmd!(music, loop_cmd),
        cmd!(music, volume),
        cmd!(music, info, nowplaying),
        // voice
        cmd!(voice, join),
        cmd!(voice, leave),
        // queue
        cmd!(queue, queue),
        cmd!(queue, skip),
        cmd!(queue, remove),
        cmd!(queue, clear),
        cmd!(queue, shuffle),
    ]
}
