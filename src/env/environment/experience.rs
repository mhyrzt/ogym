use super::terminal::Terminal;

#[derive(Debug, Clone, Copy)]
pub struct Experience<S, I, A> {
    pub curr_state: S,
    pub action: A,
    pub reward: f64,
    pub next_state: S,
    pub terminal: Terminal,
    pub info: I,
    pub step: u32,
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
