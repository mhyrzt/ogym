#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Terminal {
    Ongoing,
    Both,
    Truncate,
    Terminate,
}

impl Terminal {
    #[inline]
    pub fn is_terminated(&self) -> bool {
        matches!(self, Terminal::Terminate | Terminal::Both)
    }

    #[inline]
    pub fn is_truncated(&self) -> bool {
        matches!(self, Terminal::Truncate | Terminal::Both)
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        !matches!(self, Terminal::Ongoing)
    }

    pub fn from_flags(terminate: bool, truncate: bool) -> Self {
        match (terminate, truncate) {
            (false, false) => Terminal::Ongoing,
            (true, false) => Terminal::Terminate,
            (false, true) => Terminal::Truncate,
            (true, true) => Terminal::Both,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_ongoing() {
        let terminal = Terminal::Ongoing;
        assert!(!terminal.is_terminated());
        assert!(!terminal.is_truncated());
        assert!(!terminal.is_done());
    }

    #[test]
    fn test_terminal_terminate() {
        let terminal = Terminal::Terminate;
        assert!(terminal.is_terminated());
        assert!(!terminal.is_truncated());
        assert!(terminal.is_done());
    }

    #[test]
    fn test_terminal_truncate() {
        let terminal = Terminal::Truncate;
        assert!(!terminal.is_terminated());
        assert!(terminal.is_truncated());
        assert!(terminal.is_done());
    }

    #[test]
    fn test_terminal_both() {
        let terminal = Terminal::Both;
        assert!(terminal.is_terminated());
        assert!(terminal.is_truncated());
        assert!(terminal.is_done());
    }

    #[test]
    fn test_from_flags() {
        assert_eq!(Terminal::from_flags(false, false), Terminal::Ongoing);
        assert_eq!(Terminal::from_flags(true, false), Terminal::Terminate);
        assert_eq!(Terminal::from_flags(false, true), Terminal::Truncate);
        assert_eq!(Terminal::from_flags(true, true), Terminal::Both);
    }

    #[test]
    fn test_derive_traits() {
        let t1 = Terminal::Ongoing;
        let t2 = t1;
        assert_eq!(t1, t2);
        assert_eq!(format!("{:?}", t1), "Ongoing");
    }
}
