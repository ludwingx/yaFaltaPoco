use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Default for ColorRgb {
    fn default() -> Self {
        Self {
            r: 37,
            g: 99,
            b: 235,
        }
    }
}

impl ColorRgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub const DEFAULT_COLORS: [(&str, ColorRgb); 8] = [
    ("Azul Océano", ColorRgb::new(37, 99, 235)),
    ("Cian Eléctrico", ColorRgb::new(34, 211, 238)),
    ("Azul Cobalto", ColorRgb::new(29, 78, 216)),
    ("Azul Cielo", ColorRgb::new(56, 189, 248)),
    ("Ámbar", ColorRgb::new(245, 158, 11)),
    ("Verde", ColorRgb::new(16, 185, 129)),
    ("Violeta", ColorRgb::new(147, 51, 234)),
    ("Gris", ColorRgb::new(100, 116, 139)),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeekdayFilter {
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
}

impl Default for WeekdayFilter {
    fn default() -> Self {
        Self::monday_to_saturday()
    }
}

impl WeekdayFilter {
    pub const fn monday_to_saturday() -> Self {
        Self {
            monday: true,
            tuesday: true,
            wednesday: true,
            thursday: true,
            friday: true,
            saturday: true,
            sunday: false,
        }
    }

    pub const fn all() -> Self {
        Self {
            monday: true,
            tuesday: true,
            wednesday: true,
            thursday: true,
            friday: true,
            saturday: true,
            sunday: true,
        }
    }

    pub fn matches(&self, weekday: Weekday) -> bool {
        match weekday {
            Weekday::Mon => self.monday,
            Weekday::Tue => self.tuesday,
            Weekday::Wed => self.wednesday,
            Weekday::Thu => self.thursday,
            Weekday::Fri => self.friday,
            Weekday::Sat => self.saturday,
            Weekday::Sun => self.sunday,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimeBlock {
    pub id: String,
    pub title: String,
    pub start_hour: u32,
    pub start_min: u32,
    pub end_hour: u32,
    pub end_min: u32,
    pub color: ColorRgb,
    pub days: WeekdayFilter,
    pub notify_on_start: bool,
    pub notify_on_end: bool,
    pub notify_before_min: Option<u32>,
}

impl TimeBlock {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        start_hour: u32,
        start_min: u32,
        end_hour: u32,
        end_min: u32,
        color: ColorRgb,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            start_hour,
            start_min,
            end_hour,
            end_min,
            color,
            days: WeekdayFilter::monday_to_saturday(),
            notify_on_start: true,
            notify_on_end: true,
            notify_before_min: Some(5),
        }
    }

    pub fn start_time(&self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.start_hour, self.start_min, 0)
            .unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).expect("valid time"))
    }

    pub fn end_time(&self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.end_hour, self.end_min, 0)
            .unwrap_or_else(|| NaiveTime::from_hms_opt(23, 59, 59).expect("valid time"))
    }

    pub fn crosses_midnight(&self) -> bool {
        let start = self.start_hour * 60 + self.start_min;
        let end = self.end_hour * 60 + self.end_min;
        end <= start
    }

    pub fn interval_for_date(&self, date: NaiveDate) -> (NaiveDateTime, NaiveDateTime) {
        let start_dt = date.and_time(self.start_time());
        let end_dt = if self.crosses_midnight() {
            let next_day = date.succ_opt().unwrap_or(date);
            next_day.and_time(self.end_time())
        } else {
            date.and_time(self.end_time())
        };
        (start_dt, end_dt)
    }

    pub fn is_active_at(&self, now: NaiveDateTime) -> bool {
        let today = now.date();
        if self.days.matches(today.weekday()) {
            let (start, end) = self.interval_for_date(today);
            if now >= start && now < end {
                return true;
            }
        }
        if self.crosses_midnight() {
            let yesterday = today.pred_opt().unwrap_or(today);
            if self.days.matches(yesterday.weekday()) {
                let (start, end) = self.interval_for_date(yesterday);
                if now >= start && now < end {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_completed_at(&self, now: NaiveDateTime) -> bool {
        let today = now.date();
        if self.days.matches(today.weekday()) {
            let (_, end) = self.interval_for_date(today);
            if now >= end {
                return true;
            }
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScheduleStatus<'a> {
    Active {
        block: &'a TimeBlock,
        remaining_secs: i64,
        progress: f32,
    },
    Between {
        next_block: &'a TimeBlock,
        starts_in_secs: i64,
    },
    DayFinished,
    FreeDay,
    Empty,
}

pub fn default_sample_blocks() -> Vec<TimeBlock> {
    vec![
        TimeBlock::new("block-1", "OtherBrain I", 8, 0, 12, 0, ColorRgb::new(37, 99, 235)), // Azul
        TimeBlock::new("block-2", "Almuerzo", 12, 0, 13, 0, ColorRgb::new(245, 158, 11)),   // Ámbar
        TimeBlock::new("block-3", "OtherBrain II", 13, 0, 20, 0, ColorRgb::new(37, 99, 235)), // Azul (concorde con el degradado azul)
    ]
}

pub fn evaluate_schedule<'a>(blocks: &'a [TimeBlock], now: NaiveDateTime) -> ScheduleStatus<'a> {
    if blocks.is_empty() {
        return ScheduleStatus::Empty;
    }

    let today = now.date();
    let has_blocks_today = blocks.iter().any(|b| b.days.matches(today.weekday()));
    if !has_blocks_today {
        return ScheduleStatus::FreeDay;
    }

    // 1. Bloque activo ahora
    for block in blocks {
        if block.is_active_at(now) {
            let yesterday = today.pred_opt().unwrap_or(today);
            let (start, end) = if block.crosses_midnight() && now.time() < block.end_time() {
                block.interval_for_date(yesterday)
            } else {
                block.interval_for_date(today)
            };

            let total = (end - start).num_seconds().max(1);
            let elapsed = (now - start).num_seconds().clamp(0, total);
            let remaining = (end - now).num_seconds().max(0);
            let progress = (elapsed as f32 / total as f32).clamp(0.0, 1.0);

            return ScheduleStatus::Active {
                block,
                remaining_secs: remaining,
                progress,
            };
        }
    }

    // 2. Siguiente bloque hoy
    let mut upcoming_today: Vec<(&TimeBlock, i64)> = Vec::new();
    for block in blocks {
        if block.days.matches(today.weekday()) {
            let (start, _) = block.interval_for_date(today);
            if start > now {
                let diff = (start - now).num_seconds();
                upcoming_today.push((block, diff));
            }
        }
    }

    if !upcoming_today.is_empty() {
        upcoming_today.sort_by_key(|&(_, diff)| diff);
        let (next_block, starts_in) = upcoming_today[0];
        return ScheduleStatus::Between {
            next_block,
            starts_in_secs: starts_in,
        };
    }

    ScheduleStatus::DayFinished
}

pub fn format_duration_hms(secs: i64) -> String {
    let s = secs.max(0);
    let hours = s / 3600;
    let mins = (s % 3600) / 60;
    let seconds = s % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, seconds)
}
