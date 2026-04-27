use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{client::Context, commands::voice::get_user_voice_channel, error::BotError};
use dashmap::DashMap;
use poise::serenity_prelude;
use songbird::{
    CoreEvent, Event, EventContext, EventHandler,
    model::{id::UserId, payload::Speaking},
    packet::Packet,
};
use tracing::instrument;

/// Join a voice channel
#[poise::command(slash_command, guild_only)]
#[instrument(skip(ctx), fields(guild = ?ctx.guild_id()))]
pub async fn join(
    ctx: Context<'_>,
    #[description = "Voice channel to join (defaults to yours)"] channel: Option<
        serenity_prelude::Channel,
    >,
) -> Result<(), BotError> {
    let guild_id = ctx
        .guild_id()
        .ok_or(BotError::Internal("Not in guild".into()))?;

    let channel_id = match channel {
        Some(ch) => ch.id(),
        None => get_user_voice_channel(ctx)?,
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(BotError::Internal("Songbird not initialized".into()))?;

    {
        let handler_lock = manager.get_or_insert(guild_id);
        let mut handler = handler_lock.lock().await;

        let evt_receiver = Receiver::new();

        // handler.add_global_event(CoreEvent::SpeakingStateUpdate.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::RtpPacket.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::RtcpPacket.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::ClientDisconnect.into(), evt_receiver.clone());
        // handler.add_global_event(CoreEvent::VoiceTick.into(), evt_receiver);
    }

    if let Err(e) = manager.join(guild_id, channel_id).await {
        tracing::error!("join error: {:?}", e);
        return Err(BotError::SongbirdJoin(e));
    }

    tracing::info!(%guild_id, %channel_id, "Joined voice channel");
    ctx.say(format!("Joined <#{channel_id}>")).await?;

    Ok(())
}

#[derive(Clone)]
struct Receiver {
    inner: Arc<InnerReceiver>,
}

struct InnerReceiver {
    last_tick_was_empty: AtomicBool,
    known_ssrcs: DashMap<u32, UserId>,
}

impl Receiver {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(InnerReceiver {
                last_tick_was_empty: AtomicBool::default(),
                known_ssrcs: DashMap::new(),
            }),
        }
    }
}

#[serenity::async_trait]
impl EventHandler for Receiver {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        use EventContext as Ctx;

        match ctx {
            Ctx::SpeakingStateUpdate(Speaking {
                user_id,
                speaking,
                ssrc,
                ..
            }) => {
                // Discord voice calls use RTP, where every sender uses a randomly allocated
                // *Synchronisation Source* (SSRC) to allow receivers to tell which audio
                // stream a received packet belongs to. As this number is not derived from
                // the sender's user_id, only Discord Voice Gateway messages like this one
                // inform us about which random SSRC a user has been allocated. Future voice
                // packets will contain *only* the SSRC.
                //
                // You can implement logic here so that you can differentiate users'
                // SSRCs and map the SSRC to the User ID and maintain this state.
                // Using this map, you can map the `ssrc` in `voice_packet`
                // to the user ID and handle their audio packets separately.
                tracing::info!(
                    "Speaking state update: user {:?} has SSRC {:?}, using {:?}",
                    user_id,
                    ssrc,
                    speaking
                );

                if let Some(user) = user_id {
                    self.inner.known_ssrcs.insert(*ssrc, *user);
                }
            }
            Ctx::VoiceTick(tick) => {
                let speaking = tick.speaking.len();
                let total_participants = speaking + tick.silent.len();
                let last_tick_was_empty = self.inner.last_tick_was_empty.load(Ordering::SeqCst);

                if speaking == 0 && !last_tick_was_empty {
                    tracing::info!("No speakers");

                    self.inner.last_tick_was_empty.store(true, Ordering::SeqCst);
                } else if speaking != 0 {
                    self.inner
                        .last_tick_was_empty
                        .store(false, Ordering::SeqCst);

                    tracing::info!("Voice tick ({speaking}/{total_participants} live):");

                    for (ssrc, data) in &tick.speaking {
                        let user_id_str = if let Some(id) = self.inner.known_ssrcs.get(ssrc) {
                            format!("{:?}", *id)
                        } else {
                            "?".into()
                        };

                        if let Some(decoded_voice) = data.decoded_voice.as_ref() {
                            let voice_len = decoded_voice.len();
                            let audio_str = format!(
                                "first samples from {}: {:?}",
                                voice_len,
                                &decoded_voice[..voice_len.min(5)]
                            );

                            if let Some(packet) = &data.packet {
                                let rtp = packet.rtp();
                                tracing::info!(
                                    "\t{ssrc}/{user_id_str}: packet seq {} ts {} -- {audio_str}",
                                    rtp.get_sequence().0,
                                    rtp.get_timestamp().0
                                );
                            } else {
                                tracing::info!(
                                    "\t{ssrc}/{user_id_str}: Missed packet -- {audio_str}",
                                );
                            }
                        } else {
                            tracing::info!("\t{ssrc}/{user_id_str}: Decode disabled",)
                        }
                    }
                }
            }
            Ctx::RtpPacket(packet) => {
                // An event which fires for every received audio packet,
                // containing the decoded data.
                let rtp = packet.rtp();
                tracing::info!(
                    "Received voice packet from SSRC {}, sequence {}, timestamp {} -- {}B long",
                    rtp.get_ssrc(),
                    rtp.get_sequence().0,
                    rtp.get_timestamp().0,
                    rtp.payload().len()
                );
            }
            Ctx::RtcpPacket(data) => {
                // An event which fires for every received rtcp packet,
                // containing the call statistics and reporting information.
                tracing::info!("RTCP packet received: {:?}", data.packet);
            }
            _ => {}
        }

        None
    }
}
