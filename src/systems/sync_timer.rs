use rblitz_packets::packets::game::server::SSyncSimTime;
use shred::{Read, ReadExpect, System};

use crate::{
    packet::{dispatcher_sys::PacketSender, Channel},
    world::resources::{GameState, Time},
    TICKS_PER_SECOND,
};

const SLEEP_TIME_IN_TICKS: u32 = TICKS_PER_SECOND as u32 * 10;

pub struct SyncTimer(u32);

impl Default for SyncTimer {
    fn default() -> Self {
        SyncTimer(1)
    }
}

impl<'a> System<'a> for SyncTimer {
    type SystemData = (ReadExpect<'a, Time>, Read<'a, GameState>, PacketSender<'a>);

    fn run(&mut self, (time, state, sender): Self::SystemData) {
        match *state {
            GameState::Paused | GameState::Running => {
                self.0 -= 1;
                if self.0 == 0 {
                    self.0 = SLEEP_TIME_IN_TICKS;
                    sender.gp_broadcast_all(
                        Channel::Broadcast,
                        0,
                        &SSyncSimTime {
                            sync_time: time.game_time as f32,
                        },
                    );
                }
            },
            GameState::Loading => (),
        }
    }
}
