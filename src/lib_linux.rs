extern crate libc;

use std::collections::HashMap;
use std::io::Read;
use std::io::Result;
use std::thread;

pub fn thread_self() -> libc::pid_t {
    unsafe { libc::syscall(libc::SYS_gettid) as libc::pid_t }
}

pub fn ticks_per_second() -> u64 {
    unsafe { libc::sysconf(libc::_SC_CLK_TCK) as u64 }
}

pub fn get_thread_times(task_id: libc::pid_t, tps: u64) -> Result<(u64, u64)> {
    std::fs::OpenOptions::new()
        .read(true)
        .open(format!("/proc/self/task/{}/stat", task_id))
        .and_then(|mut file| {
            // Read by lines, then grab the ones we care about.
            let mut contents = String::new();
            file.read_to_string(&mut contents);
            // This is actually not robust, because the task name can contain both parens and
            // spaces, so it needs more intelligent parsing.
            let mut pieces = contents.split(' ');
            let utime = pieces.nth(13).unwrap().parse::<u64>().unwrap() / tps;
            let stime = pieces.nth(0).unwrap().parse::<u64>().unwrap() / tps;
            Ok((utime, stime))
        })
}

pub fn get_all_thread_times(tps: u64) -> Result<HashMap<i32, Result<(u64, u64)>>> {
    std::fs::read_dir("/proc/self/task").and_then(|mut entries| {
        let mut map = HashMap::new();
        for entry in entries {
            let file = entry.unwrap().file_name();
            // Probably faster to send the get_thread thing a direct string from `path()`.
            let tid = file.to_str().unwrap().parse::<i32>().unwrap();
            map.insert(tid, get_thread_times(tid, tps));
        }

        Ok(map)
    })
}

#[cfg(test)]
mod test {
    use super::{get_all_thread_times, get_thread_times, thread_self, ticks_per_second};
    use std::io::Write;

    #[test]
    fn test_times() {
        // accumulate some time
        let mut devnull = std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/null")
            .unwrap();
        for i in 0..1000 {
            writeln!(devnull, "{}", (1..100000).sum::<u64>());
        }
        println!(
            "{:?}",
            get_thread_times(thread_self(), ticks_per_second()).expect("times")
        );
    }

    #[test]
    fn test_times_one() {
        // accumulate some time
        let mut devnull = std::fs::OpenOptions::new()
            .write(true)
            .open("/dev/null")
            .unwrap();
        for i in 0..1000 {
            writeln!(devnull, "{}", (1..100000).sum::<u64>());
        }
        println!(
            "{:?}",
            get_all_thread_times(ticks_per_second()).expect("times")
        );
    }
}
