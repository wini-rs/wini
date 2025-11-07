use std::time::Duration;

/// Trait to implement human readable syntax for duration declaration
pub trait HumanDuration {
    fn seconds(self) -> Duration;
    fn minutes(self) -> Duration;
    fn hours(self) -> Duration;
    fn days(self) -> Duration;
}

impl HumanDuration for u64 {
    fn seconds(self) -> Duration {
        Duration::from_secs(self)
    }

    fn minutes(self) -> Duration {
        Duration::from_secs(self * 60)
    }

    fn hours(self) -> Duration {
        Duration::from_secs(self * 60 * 60)
    }

    fn days(self) -> Duration {
        Duration::from_secs(self * 60 * 60 * 24)
    }
}
