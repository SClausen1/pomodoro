extern crate pomodoro;
use pomodoro::Timer;
use std::thread::sleep;
use std::time::Duration;

fn answer() -> i32 {
  42
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

#[test]
fn check_timer_monotonic() {
  let timer = Timer::new(10);
  sleep(Duration::from_millis(50));
  assert!(timer.get_elapsed_time() >= Duration::from_millis(50));
  assert!(timer.get_minutes_remaining() == 10);
}
