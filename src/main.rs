use std::io;
use std::time::{Duration, Instant};

struct TapSeries {
    last: Instant,
    total_us: u64,
    count: u64
}

impl TapSeries {
    fn bpm(&self) -> f64 {
        if self.count <= 1 {
            return 0.0;
        }
        let avg_beat_period = (self.total_us / (self.count - 1)) as f64;
        // println!("{} {} avg period: {}", self.total_us, self.count, avg_beat_period);
        let us_per_minute = 60_000_000;
        f64::from(us_per_minute) / avg_beat_period
    }
    fn desc(&self) -> String {
        let bpm = self.bpm();
        if bpm == 0.0 {
            return String::from("Tap!");
        }
        return format!("{:.2}", bpm);
    }
    fn tap(&mut self) -> Duration {
        if self.count == 0 {
            self.last = Instant::now();
            self.count = 1;
            self.total_us = 0;
            return self.last.duration_since(self.last);
        }
        let i = Instant::now();
        let d = i.duration_since(self.last);
        self.total_us += d.as_secs() * 1_000_000;
        self.total_us += u64::from(d.subsec_nanos() / 1_000);
        self.count += 1;
        self.last = i;
        return d;
    }
    fn new() -> TapSeries {
        TapSeries{ last: Instant::now(), total_us: 0, count: 0 }
    }
}

fn main() {
    let timeout = Duration::from_secs(2);
    let mut ts = TapSeries::new();

    loop {
        println!("{}", ts.desc());
        let  mut stdin = String::from("");
        match io::stdin().read_line(&mut stdin) {
            Ok(_n) => {
                let d = ts.tap();
                if d > timeout {
                    ts = TapSeries::new();
                    ts.tap();
                }
            }
            Err(e) => {
                println!("Hrm: {}", e);
                break;
            }
        }
    }
}
