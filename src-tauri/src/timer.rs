use chrono::{NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimpleTimer {
    pub target_hour: u32,
    pub target_min: u32,
    pub state: TimerState,
    pub paused_remaining_secs: Option<i64>,
}

impl Default for SimpleTimer {
    fn default() -> Self {
        Self {
            target_hour: 18,
            target_min: 0,
            state: TimerState::Stopped,
            paused_remaining_secs: None,
        }
    }
}

pub fn calculate_target_datetime(target_time: NaiveTime, now: NaiveDateTime) -> NaiveDateTime {
    let today = now.date();
    let today_target = today.and_time(target_time);
    if now < today_target {
        today_target
    } else {
        let tomorrow = today.succ_opt().unwrap_or(today);
        tomorrow.and_time(target_time)
    }
}

pub fn calculate_remaining_seconds(target_time: NaiveTime, now: NaiveDateTime) -> (i64, bool) {
    let target = calculate_target_datetime(target_time, now);
    let diff = (target - now).num_seconds();
    if diff <= 0 {
        (0, true)
    } else {
        (diff, false)
    }
}

impl SimpleTimer {
    pub fn target_time(&self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.target_hour, self.target_min, 0)
            .unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"))
    }

    #[allow(dead_code)]
    pub fn start(&mut self) {
        self.state = TimerState::Running;
        self.paused_remaining_secs = None;
    }

    #[allow(dead_code)]
    pub fn pause(&mut self, current_remaining: i64) {
        if self.state == TimerState::Running {
            self.state = TimerState::Paused;
            self.paused_remaining_secs = Some(current_remaining);
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.state = TimerState::Stopped;
        self.paused_remaining_secs = None;
    }

    pub fn evaluate(&self, now: NaiveDateTime) -> (i64, bool) {
        match self.state {
            TimerState::Stopped => {
                let (rem, _) = calculate_remaining_seconds(self.target_time(), now);
                (rem, false)
            }
            TimerState::Paused => {
                let rem = self.paused_remaining_secs.unwrap_or(0);
                (rem, false)
            }
            TimerState::Running => calculate_remaining_seconds(self.target_time(), now),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_in_future_today() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let now = date.and_hms_opt(14, 0, 0).unwrap();
        let target = NaiveTime::from_hms_opt(18, 0, 0).unwrap();

        let (rem, triggered) = calculate_remaining_seconds(target, now);
        assert_eq!(rem, 4 * 3600);
        assert!(!triggered);
    }

    #[test]
    fn test_target_already_passed_today_sets_for_tomorrow() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let now = date.and_hms_opt(19, 0, 0).unwrap();
        let target = NaiveTime::from_hms_opt(18, 0, 0).unwrap();

        let target_dt = calculate_target_datetime(target, now);
        let expected_date = NaiveDate::from_ymd_opt(2026, 11, 6).unwrap();
        assert_eq!(target_dt.date(), expected_date);
        assert_eq!(target_dt.time(), target);

        let (rem, triggered) = calculate_remaining_seconds(target, now);
        assert_eq!(rem, 23 * 3600);
        assert!(!triggered);
    }

    #[test]
    fn test_exact_time_matches() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let now = date.and_hms_opt(18, 0, 0).unwrap();
        let target = NaiveTime::from_hms_opt(18, 0, 0).unwrap();

        let target_dt = calculate_target_datetime(target, now);
        assert_eq!(target_dt.date(), date.succ_opt().unwrap());
    }

    #[test]
    fn test_resume_after_system_sleep_jump() {
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let wake_time = date.and_hms_opt(18, 5, 0).unwrap();

        let target_dt_orig = date.and_hms_opt(18, 0, 0).unwrap();
        let diff = (target_dt_orig - wake_time).num_seconds();
        assert!(diff < 0, "Al despertar la hora objetivo ya pasó");
    }
}
