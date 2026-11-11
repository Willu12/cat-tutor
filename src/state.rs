use crate::sprites::Frame;

pub struct State {
    idle_since_ms: f64,
    waiting_for_result: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Run,
    Submit,
    Result,
}

const LYING_AFTER_MS: f64 = 60_000.;
const SLEEPING_AFTER_MS: f64 = 180_000.;

impl State {
    pub fn new(now_ms: f64) -> State {
        State {
            idle_since_ms: now_ms,
            waiting_for_result: false,
        }
    }

    pub fn frame(&self, now_ms: f64) -> Frame {
        let time_diff_ms = now_ms - self.idle_since_ms;

        match (self.waiting_for_result, time_diff_ms) {
            (true, _) => Frame::Shouting,
            (false, ..LYING_AFTER_MS) => Frame::Idle,
            (false, LYING_AFTER_MS..SLEEPING_AFTER_MS) => Frame::Lying,
            (false, _) => Frame::Sleeping,
        }
    }

    pub fn handle(&mut self, event: Event, now_ms: f64) {
        match event {
            Event::Submit | Event::Run => {
                self.waiting_for_result = true;
            }
            Event::Result => {
                self.waiting_for_result = false;
                self.idle_since_ms = now_ms;
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page opened at t = 1000 ms, so the tests don't rely on the start being 0.
    const START_MS: f64 = 1_000.0;

    #[test]
    fn idle_right_after_start() {
        let state = State::new(START_MS);
        assert_eq!(state.frame(START_MS), Frame::Idle);
    }

    #[test]
    fn idle_just_before_lying_threshold() {
        let state = State::new(START_MS);
        assert_eq!(state.frame(START_MS + LYING_AFTER_MS - 1.0), Frame::Idle);
    }

    #[test]
    fn lying_exactly_at_threshold() {
        let state = State::new(START_MS);
        assert_eq!(state.frame(START_MS + LYING_AFTER_MS), Frame::Lying);
    }

    #[test]
    fn lying_just_before_sleeping_threshold() {
        let state = State::new(START_MS);
        assert_eq!(
            state.frame(START_MS + SLEEPING_AFTER_MS - 1.0),
            Frame::Lying
        );
    }

    #[test]
    fn sleeping_exactly_at_threshold() {
        let state = State::new(START_MS);
        assert_eq!(state.frame(START_MS + SLEEPING_AFTER_MS), Frame::Sleeping);
    }

    #[test]
    fn sleeping_long_after() {
        let state = State::new(START_MS);
        assert_eq!(state.frame(START_MS + 60.0 * 60_000.0), Frame::Sleeping);
    }

    #[test]
    fn shouting_while_waiting_for_result() {
        let state = State {
            idle_since_ms: START_MS,
            waiting_for_result: true,
        };
        assert_eq!(state.frame(START_MS), Frame::Shouting);
    }

    #[test]
    fn shouting_beats_elapsed_time() {
        // Waiting for the verdict wins even if the cat would otherwise sleep.
        let state = State {
            idle_since_ms: START_MS,
            waiting_for_result: true,
        };
        assert_eq!(
            state.frame(START_MS + SLEEPING_AFTER_MS * 10.0),
            Frame::Shouting
        );
    }
}
