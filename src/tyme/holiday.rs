use core::fmt::{Display, Formatter};

use alloc::string::{String, ToString};

use crate::tyme::solar::SolarDay;
use crate::tyme::Culture;

include!(concat!(env!("OUT_DIR"), "/util_data.rs"));

#[rustfmt::skip]
pub static LEGAL_HOLIDAY_NAMES: [&str; 9] = ["元旦节", "春节", "清明节", "劳动节", "端午节", "中秋节", "国庆节", "国庆中秋", "抗战胜利日"];

/// 法定假日（自2001-12-29起）
#[derive(Debug, Copy, Clone)]
pub struct LegalHoliday {
  /// 公历日
  day: SolarDay,
  /// 索引
  index: usize,
  /// 是否上班
  work: bool,
}

impl Culture for LegalHoliday {
  fn get_name(&self) -> String {
    LEGAL_HOLIDAY_NAMES[self.index].to_string()
  }
}

impl LegalHoliday {
  pub fn from_ymd(year: isize, month: usize, day: usize) -> Option<Self> {
    for (h_year, h_month, h_day, work, festival_type, _offset_days) in LEGAL_HOLIDAY_ENTRIES {
      if h_year == year && h_month == month && h_day == day {
        return Some(Self {
          day: SolarDay::from_ymd(h_year, h_month, h_day),
          index: festival_type,
          work,
        });
      }
    }
    None
  }

  pub fn get_day(&self) -> SolarDay {
    self.day
  }

  pub fn is_work(&self) -> bool {
    self.work
  }

  pub fn next(&self, n: isize) -> Option<Self> {
    if n == 0 {
      return Some(*self);
    }

    // 获取当前日期的年、月、日
    let current_year = self.day.get_year();
    let current_month = self.day.get_month();
    let current_day = self.day.get_day();

    // 在静态数组中查找当前日期对应的索引
    let current_index = LEGAL_HOLIDAY_ENTRIES
      .iter()
      .position(|&(y, m, d, _, _, _)| {
        y == current_year && m == current_month && d == current_day
      })?;

    // 计算目标索引
    let new_index = current_index as isize + n;
    if new_index < 0 || new_index >= LEGAL_HOLIDAY_ENTRIES.len() as isize {
      return None;
    }

    // 获取目标条目的年月日，并构造新的假日对象
    let (year, month, day, _, _, _) = LEGAL_HOLIDAY_ENTRIES[new_index as usize];
    Self::from_ymd(year, month, day)
  }
}

impl Display for LegalHoliday {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    write!(
      f,
      "{} {}({})",
      self.get_day(),
      self.get_name(),
      if self.work { "班" } else { "休" }
    )
  }
}

impl PartialEq for LegalHoliday {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl Eq for LegalHoliday {}

#[cfg(test)]
mod tests {
  use alloc::string::ToString;

  use crate::tyme::holiday::LegalHoliday;

  #[test]
  fn test1() {
    let d: LegalHoliday = LegalHoliday::from_ymd(2011, 5, 1).unwrap();
    assert_eq!("2011年5月1日 劳动节(休)", d.to_string());
    assert_eq!("2011年5月2日 劳动节(休)", d.next(1).unwrap().to_string());
    assert_eq!("2011年6月4日 端午节(休)", d.next(2).unwrap().to_string());
    assert_eq!("2011年4月30日 劳动节(休)", d.next(-1).unwrap().to_string());
    assert_eq!("2011年4月5日 清明节(休)", d.next(-2).unwrap().to_string());
  }

  #[test]
  fn test2() {
    let d: LegalHoliday = LegalHoliday::from_ymd(2001, 12, 29).unwrap();
    assert_eq!("2001年12月29日 元旦节(班)", d.to_string());
    assert_eq!(true, d.next(-1).is_none());
  }

  #[test]
  fn test3() {
    let d: LegalHoliday = LegalHoliday::from_ymd(2022, 10, 5).unwrap();
    assert_eq!("2022年10月5日 国庆节(休)", d.to_string());
    assert_eq!("2022年10月4日 国庆节(休)", d.next(-1).unwrap().to_string());
    assert_eq!("2022年10月6日 国庆节(休)", d.next(1).unwrap().to_string());
  }

  #[test]
  fn test4() {
    let d: LegalHoliday = LegalHoliday::from_ymd(2010, 10, 1).unwrap();
    assert_eq!("2010年10月1日 国庆节(休)", d.to_string());
  }
}
