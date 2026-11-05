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
            r: 34,
            g: 211,
            b: 238,
        }
    }
}

impl ColorRgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub const DEFAULT_COLORS: [(&str, ColorRgb); 8] = [
    ("Azul Océano", ColorRgb::new(30, 144, 255)),
    ("Cian Neón", ColorRgb::new(34, 211, 238)),
    ("Ámbar / Naranja", ColorRgb::new(245, 158, 11)),
    ("Violeta Suave", ColorRgb::new(168, 85, 247)),
    ("Verde Esmeralda", ColorRgb::new(16, 185, 129)),
    ("Rosa Fucsia", ColorRgb::new(236, 72, 153)),
    ("Rojo Coral", ColorRgb::new(239, 68, 68)),
    ("Gris Pizarra", ColorRgb::new(100, 116, 139)),
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
        Self::all()
    }
}

impl WeekdayFilter {
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
            days: WeekdayFilter::all(),
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

    #[allow(dead_code)]
    pub fn duration_seconds(&self) -> i64 {
        let start = self.start_hour as i64 * 3600 + self.start_min as i64 * 60;
        let end = self.end_hour as i64 * 3600 + self.end_min as i64 * 60;
        if self.crosses_midnight() {
            (24 * 3600 - start) + end
        } else {
            end - start
        }
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
    DayFinished {
        next_block_tomorrow: Option<&'a TimeBlock>,
        starts_in_secs: Option<i64>,
    },
    Empty,
}

pub fn default_sample_blocks() -> Vec<TimeBlock> {
    vec![
        TimeBlock::new("sample-1", "Daily Meeting", 10, 0, 10, 30, ColorRgb::new(37, 99, 235)), // Azul intenso
        TimeBlock::new("sample-2", "Focus Time", 11, 0, 13, 0, ColorRgb::new(168, 85, 247)),    // Violeta vibrante
        TimeBlock::new("sample-3", "UI Demo", 13, 30, 14, 0, ColorRgb::new(34, 211, 238)),       // Cian neón
    ]
}

pub fn detect_overlaps(blocks: &[TimeBlock], date: NaiveDate) -> Vec<(usize, usize)> {
    let mut active_today: Vec<(usize, NaiveDateTime, NaiveDateTime)> = Vec::new();
    for (idx, b) in blocks.iter().enumerate() {
        if b.days.matches(date.weekday()) {
            let (start, end) = b.interval_for_date(date);
            active_today.push((idx, start, end));
        }
    }

    let mut overlaps = Vec::new();
    for i in 0..active_today.len() {
        for j in (i + 1)..active_today.len() {
            let (_, s1, e1) = active_today[i];
            let (_, s2, e2) = active_today[j];
            if s1 < e2 && s2 < e1 {
                overlaps.push((active_today[i].0, active_today[j].0));
            }
        }
    }
    overlaps
}

pub fn evaluate_schedule<'a>(blocks: &'a [TimeBlock], now: NaiveDateTime) -> ScheduleStatus<'a> {
    if blocks.is_empty() {
        return ScheduleStatus::Empty;
    }

    let today = now.date();
    let tomorrow = today.succ_opt().unwrap_or(today);

    // 1. Verificar si hay un bloque activo ahora
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

    // 2. Si no hay bloque activo, buscar el siguiente bloque de hoy
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

    // 3. Ya pasaron los de hoy -> Buscar el primer bloque de mañana
    let mut upcoming_tomorrow: Vec<(&TimeBlock, i64)> = Vec::new();
    for block in blocks {
        if block.days.matches(tomorrow.weekday()) {
            let (start, _) = block.interval_for_date(tomorrow);
            let diff = (start - now).num_seconds();
            if diff > 0 {
                upcoming_tomorrow.push((block, diff));
            }
        }
    }

    if !upcoming_tomorrow.is_empty() {
        upcoming_tomorrow.sort_by_key(|&(_, diff)| diff);
        let (first_tomorrow, starts_in) = upcoming_tomorrow[0];
        ScheduleStatus::DayFinished {
            next_block_tomorrow: Some(first_tomorrow),
            starts_in_secs: Some(starts_in),
        }
    } else {
        ScheduleStatus::DayFinished {
            next_block_tomorrow: None,
            starts_in_secs: None,
        }
    }
}

pub fn format_duration_hms(secs: i64) -> String {
    let s = secs.max(0);
    let hours = s / 3600;
    let mins = (s % 3600) / 60;
    let seconds = s % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_blocks() -> Vec<TimeBlock> {
        default_sample_blocks()
    }

    #[test]
    fn test_active_block_first() {
        let blocks = sample_blocks();
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let now = date.and_hms_opt(9, 30, 0).unwrap();

        match evaluate_schedule(&blocks, now) {
            ScheduleStatus::Active { block, remaining_secs, progress } => {
                assert_eq!(block.title, "OtherBrain I");
                assert_eq!(remaining_secs, 2 * 3600 + 30 * 60);
                assert!((progress - 0.375).abs() < 0.01);
            }
            _ => panic!("Expected active block"),
        }
    }

    #[test]
    fn test_between_blocks() {
        let blocks = vec![
            TimeBlock::new("1", "A", 9, 0, 10, 0, ColorRgb::new(0, 0, 0)),
            TimeBlock::new("2", "B", 11, 0, 12, 0, ColorRgb::new(0, 0, 0)),
        ];
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let now = date.and_hms_opt(10, 15, 0).unwrap();

        match evaluate_schedule(&blocks, now) {
            ScheduleStatus::Between { next_block, starts_in_secs } => {
                assert_eq!(next_block.title, "B");
                assert_eq!(starts_in_secs, 45 * 60);
            }
            _ => panic!("Expected between blocks"),
        }
    }

    #[test]
    fn test_day_finished_next_tomorrow() {
        let blocks = sample_blocks();
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let now = date.and_hms_opt(21, 0, 0).unwrap();

        match evaluate_schedule(&blocks, now) {
            ScheduleStatus::DayFinished { next_block_tomorrow, starts_in_secs } => {
                assert_eq!(next_block_tomorrow.unwrap().title, "OtherBrain I");
                assert_eq!(starts_in_secs.unwrap(), 11 * 3600);
            }
            _ => panic!("Expected day finished"),
        }
    }

    #[test]
    fn test_midnight_crossing_block() {
        let blocks = vec![
            TimeBlock::new("night", "Turno Noche", 22, 0, 2, 0, ColorRgb::new(255, 0, 0)),
        ];
        let day1 = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let day2 = NaiveDate::from_ymd_opt(2026, 11, 6).unwrap();

        let now1 = day1.and_hms_opt(23, 0, 0).unwrap();
        match evaluate_schedule(&blocks, now1) {
            ScheduleStatus::Active { block, remaining_secs, .. } => {
                assert_eq!(block.title, "Turno Noche");
                assert_eq!(remaining_secs, 3 * 3600);
            }
            _ => panic!("Expected active before midnight"),
        }

        let now2 = day2.and_hms_opt(1, 0, 0).unwrap();
        match evaluate_schedule(&blocks, now2) {
            ScheduleStatus::Active { block, remaining_secs, .. } => {
                assert_eq!(block.title, "Turno Noche");
                assert_eq!(remaining_secs, 1 * 3600);
            }
            _ => panic!("Expected active after midnight"),
        }
    }

    #[test]
    fn test_overlap_detection() {
        let blocks = vec![
            TimeBlock::new("1", "A", 10, 0, 12, 0, ColorRgb::new(0, 0, 0)),
            TimeBlock::new("2", "B", 11, 0, 13, 0, ColorRgb::new(0, 0, 0)),
            TimeBlock::new("3", "C", 14, 0, 15, 0, ColorRgb::new(0, 0, 0)),
        ];
        let date = NaiveDate::from_ymd_opt(2026, 11, 5).unwrap();
        let overlaps = detect_overlaps(&blocks, date);
        assert_eq!(overlaps.len(), 1);
        assert_eq!(overlaps[0], (0, 1));
    }
}
