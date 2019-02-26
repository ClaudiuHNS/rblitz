use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameState {
    Paused,
    Running,
    Loading,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Loading
    }
}

//FIXME, add absolute delta time cause of pause we cant use Duration for it
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time {
    pub delta_time: f64,
    pub delta_duration: Duration,
    pub game_time: f64,
    pub start_instant: Instant,
}

impl Time {
    #[inline]
    pub fn new() -> Self {
        Time {
            delta_time: 0.0,
            delta_duration: Duration::new(0, 0),
            game_time: 0.0,
            start_instant: Instant::now(),
        }
    }

    #[inline]
    pub fn set_delta(&mut self, delta: Duration, game_paused: bool) -> f64 {
        self.delta_time = duration_to_f64(delta);
        if !game_paused {
            self.game_time += self.delta_time;
        }
        self.delta_duration = delta;
        self.delta_time
    }
}

#[inline]
fn duration_to_f64(elapsed: Duration) -> f64 {
    (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64) / 1_000_000_000.0
}
