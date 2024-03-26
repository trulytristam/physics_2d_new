#[derive(Clone)]
pub struct EngineTime {
    pub time_last_frame: std::time::Duration,
    pub time_start: std::time::Instant,
    pub time_frame_start: std::time::Instant,
}

impl EngineTime {
    pub fn default() -> Self {
        EngineTime {
            time_last_frame: std::time::Duration::default(),
            time_start: std::time::Instant::now(),
            time_frame_start: std::time::Instant::now(),
        }
    }
    pub fn get_delta_time(&self) -> f64 {
        return self.time_last_frame.as_secs_f64();
    }
    pub fn frame_start(&mut self) {
        self.time_last_frame = std::time::Instant::now() - self.time_frame_start;
        self.time_frame_start = std::time::Instant::now();
    }
    pub fn time_since_start(&self) -> std::time::Duration {
        std::time::Instant::now() - self.time_start
    }
}
