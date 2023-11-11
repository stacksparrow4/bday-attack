pub struct Progress {
    curr: usize,
    curr_percent: usize,
    total: usize,
}

impl Progress {
    pub fn new(total: usize) -> Self {
        Self {
            curr: 0,
            curr_percent: 0,
            total,
        }
    }

    pub fn increment(&mut self) {
        self.curr += 1;

        let percent = (100 * self.curr) / self.total;
        if percent > self.curr_percent {
            self.curr_percent = percent;
            println!("{}%", percent);
        }
    }
}
