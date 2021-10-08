use std::time::Instant;

pub trait TimeService {
    fn monotonic_time(&mut self) -> Instant {
        Instant::now()
    }
}
