use std::time::Instant;

pub(crate) struct Progress {
    curr: usize,
    curr_percent: usize,
    total: usize,
    start_time: Instant,
}

impl Progress {
    pub(crate) fn new(total: usize) -> Self {
        Self {
            curr: 0,
            curr_percent: 0,
            total,
            start_time: Instant::now(),
        }
    }

    pub(crate) fn increment(&mut self) {
        self.curr += 1;

        let percent = (100 * self.curr) / self.total;
        if percent > self.curr_percent {
            self.curr_percent = percent;
            print!("\r{}% (elapsed {:.2?})", percent, self.start_time.elapsed());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }
    }
}
