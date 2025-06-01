use super::terminal::Terminal;

pub struct Experience<S, I, A> {
    curr_state: S,
    action: A,
    reward: f64,
    next_state: S,
    terminal: Terminal,
    info: I,
    step: u32,
}

impl<S, I, A> Experience<S, I, A> {
    pub fn new(
        curr_state: S,
        reward: f64,
        action: A,
        next_state: S,
        info: I,
        terminal: Terminal,
        step: u32,
    ) -> Self {
        Self {
            curr_state,
            action,
            reward,
            next_state,
            info,
            terminal,
            step,
        }
    }
}
