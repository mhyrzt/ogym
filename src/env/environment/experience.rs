use super::terminal::Terminal;

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::environment::terminal::Terminal;

    #[test]
    fn test_experience_new() {
        let curr_state = 10;
        let action = 5;
        let reward = 1.5;
        let next_state = 11;
        let info = "some info";
        let terminal = Terminal::Ongoing;
        let step = 100;

        let exp = Experience::new(curr_state, reward, action, next_state, info, terminal, step);

        assert_eq!(exp.curr_state, 10);
        assert_eq!(exp.action, 5);
        assert_eq!(exp.reward, 1.5);
        assert_eq!(exp.next_state, 11);
        assert_eq!(exp.info, "some info");
        assert_eq!(exp.terminal, Terminal::Ongoing);
        assert_eq!(exp.step, 100);
    }

    #[test]
    fn test_experience_clone_and_copy() {
        let exp1 = Experience::new(1.0, 0.5, "action", 2.0, (), Terminal::Terminate, 1);

        let exp2 = exp1;
        let exp3 = exp1.clone();

        assert_eq!(exp1, exp2);
        assert_eq!(exp1, exp3);
        assert_eq!(exp2.curr_state, 1.0);
    }

    #[test]
    fn test_experience_debug() {
        let exp = Experience::new(0, 0.0, 0, 0, 0, Terminal::Truncate, 0);
        let debug_str = format!("{:?}", exp);
        assert!(debug_str.contains("Experience"));
        assert!(debug_str.contains("curr_state: 0"));
        assert!(debug_str.contains("terminal: Truncate"));
    }

    #[test]
    fn test_experience_equality() {
        let exp_a = Experience::new(1, 1.0, 1, 2, 1, Terminal::Ongoing, 1);
        let exp_b = Experience::new(1, 1.0, 1, 2, 1, Terminal::Ongoing, 1);
        let exp_c = Experience::new(9, 1.0, 1, 2, 1, Terminal::Ongoing, 1);

        assert_eq!(exp_a, exp_b);
        assert_ne!(exp_a, exp_c);
    }
}
