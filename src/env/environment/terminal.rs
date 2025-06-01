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