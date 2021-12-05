use std::mem;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Turn {
    Left = 0,
    _Straight = 1,
    Right = 2,
}

impl Turn {
    pub fn from_u8(n: u8) -> Option<Turn> {
        if n <= 2 {
            Some(unsafe { mem::transmute(n) })
        } else {
            None
        }
    }

    pub fn as_u8(&self) -> u8 {
        unsafe { mem::transmute(*self) }
    }
}
