use chrono::NaiveDate;
use chrono_humanize::{Accuracy, HumanTime, Tense};
use hifitime::{Duration as HiDuration, Epoch};

#[allow(dead_code)]
const DAYS_PER_CENTURY_U64: f64 = 36_525.0f64;
const NANOSECONDS_PER_MICROSECOND: f64 = 1_000.0f64;
const NANOSECONDS_PER_MILLISECOND: f64 = 1_000.0f64 * NANOSECONDS_PER_MICROSECOND;
const NANOSECONDS_PER_SECOND: f64 = 1_000.0f64 * NANOSECONDS_PER_MILLISECOND;
#[allow(dead_code)]
const NANOSECONDS_PER_SECOND_U32: f64 = 1_000_000_000.0f64;
const NANOSECONDS_PER_MINUTE: f64 = 60.0f64 * NANOSECONDS_PER_SECOND;
const NANOSECONDS_PER_HOUR: f64 = 60.0f64 * NANOSECONDS_PER_MINUTE;
const NANOSECONDS_PER_DAY: f64 = 24.0f64 * NANOSECONDS_PER_HOUR;
const NANOSECONDS_PER_WEEK: f64 = 7.0f64 * NANOSECONDS_PER_DAY;
const DAYS_IN_YEAR: f64 = 365.2425f64;
const DAYS_IN_MONTH: f64 = 30.436875f64;
const NANOSECONDS_PER_MONTH: f64 = DAYS_IN_MONTH * NANOSECONDS_PER_DAY;
const NANOSECONDS_PER_YEAR: f64 = DAYS_IN_YEAR * NANOSECONDS_PER_DAY;
#[allow(dead_code)]
const NANOSECONDS_PER_CENTURY: f64 = DAYS_PER_CENTURY_U64 * NANOSECONDS_PER_DAY;

fn main() {
    // https://www.timeanddate.com/date/timezoneduration.html?y1=2019&m1=5&d1=10&h1=21&i1=59&s1=12&y2=2023&m2=1&d2=3&h2=14&i2=32&s2=18
    // 3 years, 7 months, 23 days, 17 hours, 33 minutes, 6 seconds

    let start = NaiveDate::from_ymd_opt(2019, 5, 10)
        .unwrap()
        .and_hms_opt(21, 59, 12)
        .unwrap();
    let end = NaiveDate::from_ymd_opt(2023, 1, 3)
        .unwrap()
        .and_hms_opt(14, 32, 18)
        .unwrap();

    let time = end - start;
    println!("Time: {:?}", time);
    let ht = HumanTime::from(time);
    println!("{}", ht.to_text_en(Accuracy::Precise, Tense::Present));

    let hi_start = Epoch::from_gregorian_utc_hms(2019, 5, 10, 21, 59, 12);
    let hi_end = Epoch::from_gregorian_utc_hms(2023, 1, 3, 14, 32, 18);

    let hi_time = hi_end - hi_start;
    println!("HiTime: {:?}", hi_time);

    println!("{:?} decomposed", hi_time.decompose());
    let (sign, days, hours, minutes, seconds, ms, us, ns) = hi_time.decompose();
    println!(
        "{}{} days, {} hours, {} minutes, {} seconds, {} ms, {} us, {} ns",
        if sign == 0 { "+" } else { "-" },
        days,
        hours,
        minutes,
        seconds,
        ms,
        us,
        ns
    );

    // println!("{:?} ", (days as i64).days().to_parts());
    // println!("{} hours", hi_time.to_unit(Unit::Hour));
    let (sign, years, months, weeks, days, hours, minutes, seconds, ms, us, ns) =
        local_decompose(hi_time);
    println!(
        "{}{} years, {} months, {} weeks, {} days, {} hours, {} minutes, {} seconds, {} ms, {} us, {} ns",
        if sign == 0 { "+" } else { "-" },
        years,
        months,
        weeks,
        days,
        hours,
        minutes,
        seconds,
        ms,
        us,
        ns
    );
}

const fn div_rem_i128(me: i128, rhs: i128) -> (i128, i128) {
    (me.div_euclid(rhs), me.rem_euclid(rhs))
}

// const fn div_rem_i64(me: i64, rhs: i64) -> (i64, i64) {
//     (me.div_euclid(rhs), me.rem_euclid(rhs))
// }

fn div_rem_f64(me: f64, rhs: f64) -> (f64, f64) {
    (me.div_euclid(rhs), me.rem_euclid(rhs))
}

fn local_decompose(dur: HiDuration) -> (i8, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let sign = dur.signum();

    match dur.try_truncated_nanoseconds() {
        Ok(total_ns) => {
            let ns_left = total_ns.abs() as f64;

            let (years, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_YEAR as f64);
            let (months, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_MONTH as f64);
            let (weeks, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_WEEK as f64);
            let (days, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_DAY as f64);
            let (hours, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_HOUR as f64);
            let (minutes, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_MINUTE as f64);
            let (seconds, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_SECOND as f64);
            let (milliseconds, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_MILLISECOND as f64);
            let (microseconds, ns_left) = div_rem_f64(ns_left, NANOSECONDS_PER_MICROSECOND as f64);

            // Everything should fit in the expected types now
            (
                sign,
                years,
                months,
                weeks,
                days,
                hours,
                minutes,
                seconds,
                milliseconds,
                microseconds,
                ns_left,
            )
        }
        Err(_) => {
            // Doesn't fit on a i64, so let's use the slower i128
            let total_ns = dur.total_nanoseconds();
            let ns_left = total_ns.abs();

            let (years, ns_left) = div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_YEAR as i64));
            let (months, ns_left) = div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_MONTH as i64));
            let (weeks, ns_left) = div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_WEEK as i64));
            let (days, ns_left) = div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_DAY as i64));
            let (hours, ns_left) = div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_HOUR as i64));
            let (minutes, ns_left) =
                div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_MINUTE as i64));
            let (seconds, ns_left) =
                div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_SECOND as i64));
            let (milliseconds, ns_left) =
                div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_MILLISECOND as i64));
            let (microseconds, ns_left) =
                div_rem_i128(ns_left, i128::from(NANOSECONDS_PER_MICROSECOND as i64));

            // Everything should fit in the expected types now
            (
                sign,
                (years as f64),
                (months as f64),
                (weeks as f64),
                (days as f64),
                (hours as f64),
                (minutes as f64),
                (seconds as f64),
                (milliseconds as f64),
                (microseconds as f64),
                (ns_left as f64),
            )
        }
    }
}

// i64
// Time: Duration { secs: 115230786, nanos: 0 }
// 3 years, 7 months, 4 weeks, 16 hours, 33 minutes and 6 seconds
// HiTime: Duration { centuries: 0, nanoseconds: 115230786000000000 }
// (0, 1333, 16, 33, 6, 0, 0, 0) decomposed
// +1333 days, 16 hours, 33 minutes, 6 seconds, 0 ms, 0 us, 0 ns
// +190 weeks, 3 days, 16 hours, 33 minutes, 6 seconds, 0 ms, 0 us, 0 ns

// float
// Time: Duration { secs: 115230786, nanos: 0 }
// 3 years, 7 months, 4 weeks, 16 hours, 33 minutes and 6 seconds
// HiTime: Duration { centuries: 0, nanoseconds: 115230786000000000 }
// (0, 1333, 16, 33, 6, 0, 0, 0) decomposed
// +1333 days, 16 hours, 33 minutes, 6 seconds, 0 ms, 0 us, 0 ns
// +190 weeks, 3 days, 16 hours, 33 minutes, 6 seconds, 0 ms, 0 us, 0 ns

// Time: Duration { secs: 115230786, nanos: 0 }
// 3 years, 7 months, 4 weeks, 16 hours, 33 minutes and 6 seconds
// HiTime: Duration { centuries: 0, nanoseconds: 115230786000000000 }
// (0, 1333, 16, 33, 6, 0, 0, 0) decomposed
// +1333 days, 16 hours, 33 minutes, 6 seconds, 0 ms, 0 us, 0 ns
// +3 years, 7 months, 3 weeks, 3 days, 21 hours, 41 minutes, 48 seconds, 0 ms, 0 us, 0 ns

// 3 years, 7 months, 23 days, 17 hours, 33 minutes, 6 seconds
