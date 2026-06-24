/// according to the spec in section 6:
/// A node maintains a single sequence number that applies to all routing
/// entries originated by that node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeqNum(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// determines what to do with the sequencce number
pub enum Acceptance {
    /// keep the current seq num
    NoChange,
    /// the received seq num is fresher than the current one.
    Fresher,
    /// if the diff is greater than 100
    Reboot,
}

impl From<u16> for SeqNum {
    fn from(value: u16) -> Self {
        SeqNum(value)
    }
}

impl SeqNum {
    pub fn new() -> Self {
        SeqNum(rand::random::<u16>())
    }

    /// A sequence number `new` is fresher than `old` if and only if:
    /// (new - old) mod 65536 < 32768
    pub fn is_fresher(&self, new: SeqNum) -> bool {
        let res = new.0.wrapping_sub(self.0);
        res >= 1 && res <= 32_767
    }

    pub fn is_reboot(&self, new: SeqNum) -> bool {
        let res = new.0.wrapping_sub(self.0);
        res > 100
    }

    pub fn acceptance(&self, new: SeqNum) -> Acceptance {
        let res = new.0.wrapping_sub(self.0);
        if res == 0 {
            Acceptance::NoChange
        } else if res >= 1 && res <= 100 {
            Acceptance::Fresher
        } else {
            Acceptance::Reboot
        }
    }
}
