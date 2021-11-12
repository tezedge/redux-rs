use std::time::{Duration, SystemTime};

/// Time in nanoseconds from [std::time::UNIX_EPOCH].
///
/// Each action will have unique id. If two actions happen at the same time,
/// id must be increased by 1 for second action, to ensure uniqueness of id.
///
/// u64 is enough to contain time in nanoseconds at most 584 years
/// after `UNIX_EPOCH` (1970-01-01 00:00:00 UTC).
///
/// ```
/// //           nano     micro  milli  sec    min  hour day  year
/// assert_eq!(u64::MAX / 1000 / 1000 / 1000 / 60 / 60 / 24 / 365, 584);
/// ```
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionId(u64);

impl ActionId {
    pub const ZERO: Self = Self(0);

    /// Caller must make sure such action actually exists!
    #[inline(always)]
    pub fn new_unchecked(value: u64) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub(crate) fn next(&self, time_passed: u64) -> Self {
        Self(self.0 + time_passed.max(1))
    }

    pub fn duration_since(&self, other: ActionId) -> Duration {
        Duration::from_nanos(self.0.checked_sub(other.0).unwrap_or(0))
    }
}

impl From<ActionId> for u64 {
    fn from(id: ActionId) -> Self {
        id.0
    }
}

/// Action with additional metadata like: id.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ActionWithMeta<Action> {
    pub id: ActionId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub action: Action,
}

impl<Action> ActionWithMeta<Action> {
    #[inline(always)]
    pub fn time(&self) -> SystemTime {
        SystemTime::UNIX_EPOCH + self.duration_since_epoch()
    }

    #[inline(always)]
    pub fn time_as_nanos(&self) -> u64 {
        self.id.into()
    }

    #[inline(always)]
    pub fn duration_since_epoch(&self) -> Duration {
        Duration::from_nanos(self.time_as_nanos())
    }

    #[inline(always)]
    pub fn duration_since(&self, other: &ActionWithMeta<Action>) -> Duration {
        self.id.duration_since(other.id)
    }
}

pub trait EnablingCondition {
    type State;

    /// Enabling condition for the Action.
    ///
    /// Checks if the given action is enabled for a given state.
    fn is_enabled(&self, state: &Self::State) -> bool;
}
