use std::time::{Duration, SystemTime};
//计时器
pub struct Timer {
    frame_time: u64, //微妙
    start_time: SystemTime,
    next_time: Duration,
}

impl Timer {
    pub fn new(fps: u64) -> Timer {
        Timer {
            frame_time: 1_000_000 / fps,
            start_time: SystemTime::now(),
            next_time: Duration::from_millis(0),
        }
    }

    pub fn _start(&mut self) {
        //设置计数器起始值
        self.start_time = SystemTime::now();
        //更新时间在下一帧使用
        self.next_time = Duration::from_millis(0);
    }

    pub fn ready_for_next_frame(&mut self) -> bool {
        if self.start_time.elapsed().unwrap() > self.next_time {
            //更新时间
            self.next_time =
                self.start_time.elapsed().unwrap() + Duration::from_micros(self.frame_time);
            true
        } else {
            false
        }
    }

    //逝去的毫秒数
    pub fn elapsed_secs(&self) -> u64 {
        let duration = self.start_time.elapsed().unwrap();
        (duration.as_micros() / 1000) as u64
    }
}
