use std::time;

use indicatif::ProgressBar;

pub struct Logger {
    pb: ProgressBar,
    line_count: usize,
    token_count: usize,
    start_time: time::Instant,
    update_interval: usize,
}

impl Logger {
    pub fn new() -> Logger {
        let pb = ProgressBar::new_spinner();
        let line_count: usize = 0;
        let token_count: usize = 0;
        let start_time = time::Instant::now();
        let update_interval: usize = 100_000;
        Logger {
            pb,
            line_count,
            token_count,
            start_time,
            update_interval,
        }
    }

    pub fn tokens_per_second(&self) -> usize {
        let elapsed_nanoseconds: usize = self.start_time.elapsed().subsec_nanos() as usize;
        if elapsed_nanoseconds > 0 {
            (1_000_000_000 * self.token_count) / elapsed_nanoseconds
        } else {
            0
        }
    }

    pub fn update(&mut self, token_count: usize) {
        self.line_count += 1;
        self.token_count += token_count;
        if self.line_count % self.update_interval == 0 {
            let tokens_per_second = self.tokens_per_second();
            if tokens_per_second > 0 {
                self.pb.set_message(
                    format!(
                        "processed {} tokens ({} tokens per second)",
                        self.token_count, tokens_per_second
                    )
                    .as_str(),
                );
            } else {
                self.pb.inc(1);
            }
        }
    }

    pub fn finish(&self) {
        let tokens_per_second = self.tokens_per_second();
        if tokens_per_second > 0 {
            self.pb.finish_with_message(
                format!(
                    "processed {} tokens ({} tokens per second)",
                    self.token_count, tokens_per_second
                )
                .as_str(),
            );
        } else {
            self.pb.finish_with_message("done");
        }
    }
}
