use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

pub trait SleepResponse {
    fn get_total_duration_asleep(&self) -> chrono::Duration;
    fn get_sleep_efficiency(&self) -> Option<u8>;
    fn get_time_fell_asleep(&self) -> Option<NaiveDateTime>;
    fn get_wake_up_time(&self) -> Option<NaiveTime>;
    fn get_total_duration_awake_during_sleep(&self) -> Option<chrono::Duration>;
}

#[derive(Debug, Default, Deserialize)]
pub struct SleepResponseV1_2 {
    pub sleep: Vec<SleepData>,
    pub summary: SleepSummary,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepData {
    pub date_of_sleep: NaiveDate,
    pub duration: u64,
    pub efficiency: u8,
    pub end_time: NaiveDateTime,
    pub info_code: u8,
    pub is_main_sleep: bool,
    pub levels: SleepLevels,
    pub log_id: u64,
    pub log_type: String,
    pub minutes_after_wakeup: u32,
    pub minutes_asleep: u32,
    pub minutes_awake: u32,
    pub minutes_to_fall_asleep: u32,
    pub start_time: NaiveDateTime,
    pub time_in_bed: u32,
    #[serde(rename = "type")]
    pub sleep_type: String,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepLevels {
    pub data: Vec<LevelData>,
    pub short_data: Vec<LevelData>,
    pub summary: LevelsSummary,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SleepLevel {
    Deep,
    Light,
    Rem,
    #[default]
    Wake,
    Unknown,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelData {
    pub date_time: NaiveDateTime,
    pub level: SleepLevel,
    pub seconds: u32,
}

impl LevelData {
    pub fn is_sleep(&self) -> bool {
        self.level != SleepLevel::Wake
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct LevelsSummary {
    pub deep: LevelSummary,
    pub light: LevelSummary,
    pub rem: LevelSummary,
    pub wake: LevelSummary,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelSummary {
    pub count: u32,
    pub minutes: u32,
    pub thirty_day_avg_minutes: f32,
}

#[derive(Default, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepSummary {
    pub stages: StagesSummary,
    pub total_minutes_asleep: u32,
    pub total_sleep_records: u32,
    pub total_time_in_bed: u32,
}

#[derive(Default, Debug, Deserialize)]
pub struct StagesSummary {
    pub deep: u32,
    pub light: u32,
    pub rem: u32,
    pub wake: u32,
}

impl SleepResponse for SleepResponseV1_2 {
    fn get_total_duration_asleep(&self) -> chrono::Duration {
        chrono::Duration::minutes(self.summary.total_minutes_asleep as i64)
    }

    fn get_sleep_efficiency(&self) -> Option<u8> {
        self.sleep
            .iter()
            .find(|s| s.is_main_sleep)
            .map(|main_sleep| main_sleep.efficiency)
    }

    fn get_time_fell_asleep(&self) -> Option<NaiveDateTime> {
        const MIN_SLEEP_DURATION: u32 = 300;

        self.sleep
            .iter()
            .find(|s| s.is_main_sleep)
            .and_then(|main_sleep| {
                main_sleep
                    .levels
                    .data
                    .iter()
                    .find(|stage| stage.is_sleep() && stage.seconds > MIN_SLEEP_DURATION)
                    .map(|level_data| level_data.date_time)
            })
    }

    fn get_wake_up_time(&self) -> Option<NaiveTime> {
        self.sleep
            .iter()
            .find(|s| s.is_main_sleep)
            .map(|main_sleep| main_sleep.end_time.time())
    }

    fn get_total_duration_awake_during_sleep(&self) -> Option<chrono::Duration> {
        let main_sleep = self.sleep.iter().find(|s| s.is_main_sleep);
        if let Some(sleep) = main_sleep {
            let total_seconds: u32 = sleep.levels.data.iter().map(|level| level.seconds).sum();

            Some(chrono::Duration::seconds(total_seconds as i64))
        } else {
            None
        }
    }
}

impl SleepResponseV1_2 {
    pub fn get_time_awake_between(
        &self,
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> chrono::Duration {
        let main_sleep = self.sleep.iter().find(|s| s.is_main_sleep);
        let total_awake = end - start;

        if let Some(sleep) = main_sleep {
            let total_duration_not_awake = sleep
                .levels
                .data
                .iter()
                .filter(|level| level.level != SleepLevel::Wake && level.date_time < end)
                .fold(chrono::Duration::zero(), |acc, level| {
                    let level_start = level.date_time.max(start);
                    let level_end = (level.date_time
                        + chrono::Duration::seconds(level.seconds as i64))
                    .min(end);
                    if level_start < level_end {
                        acc + (level_end - level_start)
                    } else {
                        acc
                    }
                });

            total_awake - total_duration_not_awake
        } else {
            total_awake
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

    #[test]
    fn test_parse_real_fitbit_sleep_response() {
        // JSON data from real Fitbit response
        let json_str = r#"{
  "sleep" : [ {
    "dateOfSleep" : "2025-03-30",
    "duration" : 32220000,
    "efficiency" : 90,
    "endTime" : "2025-03-30T07:09:00.000",
    "infoCode" : 0,
    "isMainSleep" : true,
    "levels" : {
      "data" : [ {
        "dateTime" : "2025-03-29T22:11:30.000",
        "level" : "wake",
        "seconds" : 510
      }, {
        "dateTime" : "2025-03-29T22:20:00.000",
        "level" : "light",
        "seconds" : 150
      }, {
        "dateTime" : "2025-03-29T22:22:30.000",
        "level" : "wake",
        "seconds" : 300
      }, {
        "dateTime" : "2025-03-29T22:27:30.000",
        "level" : "light",
        "seconds" : 330
      }, {
        "dateTime" : "2025-03-29T22:33:00.000",
        "level" : "wake",
        "seconds" : 300
      }, {
        "dateTime" : "2025-03-29T22:38:00.000",
        "level" : "light",
        "seconds" : 540
      }, {
        "dateTime" : "2025-03-29T22:47:00.000",
        "level" : "wake",
        "seconds" : 660
      }, {
        "dateTime" : "2025-03-29T22:58:00.000",
        "level" : "light",
        "seconds" : 3570
      }, {
        "dateTime" : "2025-03-29T23:57:30.000",
        "level" : "wake",
        "seconds" : 240
      }, {
        "dateTime" : "2025-03-30T00:01:30.000",
        "level" : "light",
        "seconds" : 720
      }, {
        "dateTime" : "2025-03-30T00:13:30.000",
        "level" : "deep",
        "seconds" : 1200
      }, {
        "dateTime" : "2025-03-30T00:33:30.000",
        "level" : "light",
        "seconds" : 1920
      }, {
        "dateTime" : "2025-03-30T01:05:30.000",
        "level" : "deep",
        "seconds" : 2280
      }, {
        "dateTime" : "2025-03-30T01:43:30.000",
        "level" : "light",
        "seconds" : 1440
      }, {
        "dateTime" : "2025-03-30T02:07:30.000",
        "level" : "deep",
        "seconds" : 300
      }, {
        "dateTime" : "2025-03-30T02:12:30.000",
        "level" : "light",
        "seconds" : 960
      }, {
        "dateTime" : "2025-03-30T02:28:30.000",
        "level" : "wake",
        "seconds" : 540
      }, {
        "dateTime" : "2025-03-30T02:37:30.000",
        "level" : "light",
        "seconds" : 120
      }, {
        "dateTime" : "2025-03-30T02:39:30.000",
        "level" : "wake",
        "seconds" : 690
      }, {
        "dateTime" : "2025-03-30T02:51:00.000",
        "level" : "light",
        "seconds" : 10860
      }, {
        "dateTime" : "2025-03-30T05:52:00.000",
        "level" : "rem",
        "seconds" : 270
      }, {
        "dateTime" : "2025-03-30T05:56:30.000",
        "level" : "wake",
        "seconds" : 750
      }, {
        "dateTime" : "2025-03-30T06:09:00.000",
        "level" : "unknown",
        "seconds" : 3600
      } ],
      "shortData" : [ {
        "dateTime" : "2025-03-29T22:30:00.000",
        "level" : "wake",
        "seconds" : 60
      }, {
        "dateTime" : "2025-03-29T22:40:30.000",
        "level" : "wake",
        "seconds" : 180
      }, {
        "dateTime" : "2025-03-29T23:00:30.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T00:48:30.000",
        "level" : "wake",
        "seconds" : 60
      }, {
        "dateTime" : "2025-03-30T00:53:00.000",
        "level" : "wake",
        "seconds" : 150
      }, {
        "dateTime" : "2025-03-30T01:41:00.000",
        "level" : "wake",
        "seconds" : 150
      }, {
        "dateTime" : "2025-03-30T02:14:30.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T03:28:30.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T03:47:30.000",
        "level" : "wake",
        "seconds" : 120
      }, {
        "dateTime" : "2025-03-30T04:00:00.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T04:05:00.000",
        "level" : "wake",
        "seconds" : 60
      }, {
        "dateTime" : "2025-03-30T04:17:30.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T04:24:30.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T04:28:30.000",
        "level" : "wake",
        "seconds" : 90
      }, {
        "dateTime" : "2025-03-30T04:40:30.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T04:43:00.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T05:11:30.000",
        "level" : "wake",
        "seconds" : 30
      }, {
        "dateTime" : "2025-03-30T05:51:30.000",
        "level" : "wake",
        "seconds" : 30
      } ],
      "summary" : {
        "deep" : {
          "count" : 3,
          "minutes" : 61,
          "thirtyDayAvgMinutes" : 97
        },
        "light" : {
          "count" : 26,
          "minutes" : 326,
          "thirtyDayAvgMinutes" : 266
        },
        "rem" : {
          "count" : 1,
          "minutes" : 4,
          "thirtyDayAvgMinutes" : 67
        },
        "wake" : {
          "count" : 26,
          "minutes" : 86,
          "thirtyDayAvgMinutes" : 79
        }
      }
    },
    "logId" : 48809009246,
    "logType" : "auto_detected",
    "minutesAfterWakeup" : 0,
    "minutesAsleep" : 391,
    "minutesAwake" : 86,
    "minutesToFallAsleep" : 0,
    "startTime" : "2025-03-29T22:11:30.000",
    "timeInBed" : 537,
    "type" : "stages"
  } ],
  "summary" : {
    "stages" : {
      "deep" : 61,
      "light" : 326,
      "rem" : 4,
      "wake" : 86
    },
    "totalMinutesAsleep" : 391,
    "totalSleepRecords" : 1,
    "totalTimeInBed" : 537
  }
}"#;

        // Parse the JSON into our struct
        let response: SleepResponseV1_2 =
            serde_json::from_str(json_str).expect("Failed to parse JSON");

        // Test the sleep array
        assert_eq!(response.sleep.len(), 1, "Should have 1 sleep record");

        // Test basic sleep data fields
        let sleep = &response.sleep[0];
        assert_eq!(
            sleep.date_of_sleep,
            NaiveDate::from_ymd_opt(2025, 3, 30).unwrap()
        );
        assert_eq!(sleep.duration, 32220000);
        assert_eq!(sleep.efficiency, 90);
        assert_eq!(
            sleep.end_time,
            NaiveDateTime::parse_from_str("2025-03-30T07:09:00.000", "%Y-%m-%dT%H:%M:%S%.3f")
                .unwrap()
        );
        assert_eq!(sleep.info_code, 0);
        assert!(sleep.is_main_sleep);
        assert_eq!(sleep.log_id, 48809009246);
        assert_eq!(sleep.log_type, "auto_detected");
        assert_eq!(sleep.minutes_after_wakeup, 0);
        assert_eq!(sleep.minutes_asleep, 391);
        assert_eq!(sleep.minutes_awake, 86);
        assert_eq!(sleep.minutes_to_fall_asleep, 0);
        assert_eq!(
            sleep.start_time,
            NaiveDateTime::parse_from_str("2025-03-29T22:11:30.000", "%Y-%m-%dT%H:%M:%S%.3f")
                .unwrap()
        );
        assert_eq!(sleep.time_in_bed, 537);
        assert_eq!(sleep.sleep_type, "stages");

        // Test sleep levels data
        assert_eq!(sleep.levels.data.len(), 23);
        assert_eq!(sleep.levels.short_data.len(), 18);

        // Test first data entry
        let first_data = &sleep.levels.data[0];
        assert_eq!(
            first_data.date_time,
            NaiveDateTime::parse_from_str("2025-03-29T22:11:30.000", "%Y-%m-%dT%H:%M:%S%.3f")
                .unwrap()
        );
        assert_eq!(first_data.level, SleepLevel::Wake);
        assert_eq!(first_data.seconds, 510);

        // Test levels summary
        assert_eq!(sleep.levels.summary.deep.count, 3);
        assert_eq!(sleep.levels.summary.deep.minutes, 61);
        assert_eq!(sleep.levels.summary.deep.thirty_day_avg_minutes, 97.0);

        assert_eq!(sleep.levels.summary.light.count, 26);
        assert_eq!(sleep.levels.summary.light.minutes, 326);
        assert_eq!(sleep.levels.summary.light.thirty_day_avg_minutes, 266.0);

        assert_eq!(sleep.levels.summary.rem.count, 1);
        assert_eq!(sleep.levels.summary.rem.minutes, 4);
        assert_eq!(sleep.levels.summary.rem.thirty_day_avg_minutes, 67.0);

        assert_eq!(sleep.levels.summary.wake.count, 26);
        assert_eq!(sleep.levels.summary.wake.minutes, 86);
        assert_eq!(sleep.levels.summary.wake.thirty_day_avg_minutes, 79.0);

        // Test summary
        assert_eq!(response.summary.stages.deep, 61);
        assert_eq!(response.summary.stages.light, 326);
        assert_eq!(response.summary.stages.rem, 4);
        assert_eq!(response.summary.stages.wake, 86);
        assert_eq!(response.summary.total_minutes_asleep, 391);
        assert_eq!(response.summary.total_sleep_records, 1);
        assert_eq!(response.summary.total_time_in_bed, 537);

        // Test SleepResponse trait methods
        assert_eq!(
            response.get_total_duration_asleep(),
            chrono::Duration::minutes(391)
        );
        assert_eq!(response.get_sleep_efficiency(), Some(90));

        // Test fell asleep time - should be the first non-wake period > 300 seconds
        // The first qualifying entry is at 22:38:00 with 540 seconds of light sleep
        let expected_fell_asleep_time =
            NaiveDateTime::parse_from_str("2025-03-29T22:27:30.000", "%Y-%m-%dT%H:%M:%S%.3f")
                .unwrap();
        assert_eq!(
            response.get_time_fell_asleep(),
            Some(expected_fell_asleep_time)
        );

        // Test wake-up time
        let expected_wake_up_time = NaiveTime::parse_from_str("07:09:00", "%H:%M:%S").unwrap();
        assert_eq!(response.get_wake_up_time(), Some(expected_wake_up_time));

        // Test get_time_awake_between for a specific time range
        let start =
            NaiveDateTime::parse_from_str("2025-03-30T02:00:00.000", "%Y-%m-%dT%H:%M:%S%.3f")
                .unwrap();
        let end = NaiveDateTime::parse_from_str("2025-03-30T03:00:00.000", "%Y-%m-%dT%H:%M:%S%.3f")
            .unwrap();
        let time_awake = response.get_time_awake_between(start, end);

        // In the 02:00-03:00 range, we have wake periods:
        // - 02:28:30 for 540 seconds
        // - 02:39:30 for 690 seconds (but this goes past 03:00:00)
        // So approximately 20.5 minutes awake
        assert!(
            time_awake.num_minutes() >= 19 && time_awake.num_minutes() <= 22,
            "Expected around 20.5 minutes awake, got {} minutes",
            time_awake.num_minutes()
        );
    }
}
