use std::time::Instant;

pub struct RateLimiter {
    tokens: f64,
    capacity: f64,
    refill_rate: f64,
    last_check: Instant,
    violations: u32,
}

impl RateLimiter {
    pub fn new(capacity: f64, refill_rate: f64) -> Self {
        Self {
            tokens: capacity,
            capacity,
            refill_rate,
            last_check: Instant::now(),
            violations: 0,
        }
    }

    pub fn check(&mut self) -> Result<bool, ()> {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_check).as_secs_f64();
        self.last_check = now;

        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.capacity);

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            self.violations = 0;
            Ok(true)
        } else {
            self.violations += 1;
            if self.violations >= 10 {
                Err(())
            } else {
                Ok(false)
            }
        }
    }
}
