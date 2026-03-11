use std::env;
use std::fs;
use std::path::Path;

fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-changed=data/");

  let out_dir = env::var("OUT_DIR").unwrap();
  let out_dir = Path::new(&out_dir);
  let dest_path = out_dir.join("util_data.rs");

  let code = generate_data_code();
  fs::write(&dest_path, code).unwrap();
}

fn read_data_file(name: &str) -> String {
  let path = Path::new("data").join(name);
  fs::read_to_string(&path).unwrap_or_else(|e| {
    panic!(
      "Failed to read data file {}: {}. Please ensure the file exists in the data/ directory.",
      name, e
    );
  })
}

fn generate_leap_month_data() -> String {
  let chars: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_@";
  let data = read_data_file("leap_month_data.txt");
  let months: Vec<&str> = data.split_whitespace().collect();

  let mut list: Vec<Vec<isize>> = Vec::new();

  for m in &months {
    let mut n: isize = 0;
    let size: usize = m.len() / 2;
    let mut l: Vec<isize> = Vec::new();
    for y in 0..size {
      let z: usize = y * 2;
      let s: &str = &m[z..z + 2];
      let mut t: isize = 0;
      let mut c: isize = 1;
      let mut x: isize = 1;
      while x > -1 {
        t += c * chars.find(s.chars().nth(x as usize).unwrap()).unwrap() as isize;
        c *= 64;
        x -= 1;
      }
      n += t;
      l.push(n);
    }
    list.push(l);
  }

  let mut code = String::new();
  code.push_str("/// 闰月年份数据（由build.rs生成）\n");
  code.push_str("pub const LEAP_MONTH_YEAR: [&[isize]; 12] = [\n");

  for (i, month_data) in list.iter().enumerate() {
    code.push_str("    &[");
    for (j, val) in month_data.iter().enumerate() {
      code.push_str(&val.to_string());
      if j < month_data.len() - 1 {
        code.push_str(", ");
      }
    }
    code.push_str("], // month ");
    code.push_str(&(i + 1).to_string());
    code.push('\n');
  }

  code.push_str("];\n");
  code
}

fn generate_rab_byung_month_days_data() -> String {
  let years_str = read_data_file("rab_byung_data.txt");

  let mut entries: Vec<(usize, Vec<isize>)> = Vec::new();
  let mut y: usize = 1950;
  let mut m: usize = 11;

  for s in years_str.split(',') {
    let mut ys = s;
    while !ys.is_empty() {
      let chars: Vec<char> = ys.chars().collect();
      let len = (chars[0] as u8 - b'0') as usize;
      let mut data: Vec<isize> = Vec::new();
      for ch in chars.iter().take(len).skip(1) {
        data.push(*ch as isize - b'5' as isize - 30);
      }
      entries.push((y * 13 + m, data));
      m += 1;
      ys = &ys[1 + len..];
    }
    y += 1;
    m = 0;
  }

  let mut code = String::new();
  code.push_str("/// 藏历月数据（由 build.rs 生成）\n");
  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static RAB_BYUNG_MONTH_DAYS: [(usize, &[isize]); ");
  code.push_str(&entries.len().to_string());
  code.push_str("] = [\n");

  for (key, data) in &entries {
    code.push_str("    (");
    code.push_str(&key.to_string());
    code.push_str(", &[");
    for (i, val) in data.iter().enumerate() {
      code.push_str(&val.to_string());
      if i < data.len() - 1 {
        code.push_str(", ");
      }
    }
    code.push_str("]),\n");
  }

  code.push_str("];\n");
  code
}

fn generate_day_gods_data() -> String {
  let data = read_data_file("day_gods.txt");
  let day_gods: Vec<&str> = data.split_whitespace().collect();

  let mut lookup_table: Vec<Vec<Vec<isize>>> = vec![vec![vec![]; 60]; 12];

  for (month_idx, month_str) in day_gods.iter().enumerate() {
    let entries: Vec<&str> = month_str.trim_start_matches(';').split(';').collect();

    for entry in entries {
      if entry.len() < 2 {
        continue;
      }
      let day_hex = &entry[0..2];
      let day_idx = usize::from_str_radix(day_hex, 16).unwrap();
      let data_part = &entry[2..];

      let mut gods: Vec<isize> = Vec::new();
      for i in (0..data_part.len()).step_by(2) {
        if i + 2 <= data_part.len() {
          let idx = isize::from_str_radix(&data_part[i..i + 2], 16).unwrap();
          gods.push(idx);
        }
      }

      if day_idx < 60 {
        lookup_table[month_idx][day_idx] = gods;
      }
    }
  }

  let mut code = String::new();
  code.push_str("/// 日神煞数据（由build.rs生成）\n");
  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static DAY_GODS_LOOKUP: [&[&[isize]]; 12] = [\n");

  for (i, month_data) in lookup_table.iter().enumerate() {
    code.push_str("    &[");
    for day_gods in month_data.iter() {
      if day_gods.is_empty() {
        code.push_str("&[],");
      } else {
        code.push_str("&[");
        for (k, god_idx) in day_gods.iter().enumerate() {
          code.push_str(&god_idx.to_string());
          if k < day_gods.len() - 1 {
            code.push_str(", ");
          }
        }
        code.push_str("],");
      }
    }
    code.push_str("], // month ");
    code.push_str(&(i + 1).to_string());
    code.push('\n');
  }

  code.push_str("];\n");
  code
}

fn decode(os: &str) -> String {
  let mut s = os.to_string();
  let replacements = [
    ("J", "00"),
    ("I", "000"),
    ("H", "0000"),
    ("G", "00000"),
    ("t", "02"),
    ("s", "002"),
    ("r", "0002"),
    ("q", "00002"),
    ("p", "000002"),
    ("o", "0000002"),
    ("n", "00000002"),
    ("m", "000000002"),
    ("l", "0000000002"),
    ("k", "01"),
    ("j", "0101"),
    ("i", "001"),
    ("h", "001001"),
    ("g", "0001"),
    ("f", "00001"),
    ("e", "000001"),
    ("d", "0000001"),
    ("c", "00000001"),
    ("b", "000000001"),
    ("a", "0000000001"),
    ("A", &"0".repeat(54)),
    ("B", &"0".repeat(46)),
    ("C", &"0".repeat(37)),
    ("D", &"0".repeat(28)),
    ("E", &"0".repeat(20)),
    ("F", &"0".repeat(10)),
  ];

  for (from, to) in &replacements {
    s = s.replace(from, to);
  }
  s
}

fn generate_xl1_data() -> String {
  let data = read_data_file("xl1_data.txt");

  let mut xl1_data: Vec<Vec<f64>> = Vec::new();

  for line in data.lines() {
    if line.trim().is_empty() {
      continue;
    }
    let nums: Vec<f64> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
    if !nums.is_empty() {
      xl1_data.push(nums);
    }
  }

  let total_len: usize = xl1_data.iter().map(|v| v.len()).sum();

  let mut index_data = String::new();
  let mut offset = 0;
  for vec in &xl1_data {
    let len = vec.len();
    index_data.push_str(&format!("    ({}, {}),\n", offset, len));
    offset += len;
  }

  let xl1_flat = xl1_data
    .iter()
    .flatten()
    .map(|v| {
      if v.fract() == 0.0 {
        format!("{}.0", v)
      } else {
        format!("{:.15}", v)
          .trim_end_matches('0')
          .trim_end_matches('.')
          .to_string()
      }
    })
    .collect::<Vec<_>>()
    .join(",\n  ");

  let mut code = String::new();
  code.push_str("/// XL1 数据：月球经度计算系数（展平存储）\n");
  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static XL1_DATA: [f64; ");
  code.push_str(&total_len.to_string());
  code.push_str("] = [\n  ");
  code.push_str(&xl1_flat);
  code.push_str("\n];\n\n");

  code.push_str("/// XL1 数据索引：每个子向量的 [起始位置, 长度]\n");
  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static XL1_INDEX: [(usize, usize); ");
  code.push_str(&xl1_data.len().to_string());
  code.push_str("] = [\n");
  code.push_str(&index_data);
  code.push_str("];\n");

  code
}

fn generate_solar_festival_data() -> String {
  let data = read_data_file("solar_festival_data.txt");
  let mut entries = Vec::new();

  for entry in data.split('@').skip(1) {
    let index = entry[..2].parse::<usize>().unwrap();
    let month = entry[3..5].parse::<usize>().unwrap();
    let day = entry[5..7].parse::<usize>().unwrap();
    let start_year = entry[7..].parse::<isize>().unwrap();
    entries.push((index, month, day, start_year));
  }

  let mut code = String::new();
  code.push_str("/// 公历节日数据（由build.rs生成）\n");
  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static SOLAR_FESTIVAL_ENTRIES: [(usize, usize, usize, isize); ");
  code.push_str(&entries.len().to_string());
  code.push_str("] = [\n");

  for (index, month, day, start_year) in entries {
    code.push_str(&format!(
      "    ({}, {}, {}, {}),\n",
      index, month, day, start_year
    ));
  }

  code.push_str("];\n");
  code
}

fn generate_lunar_festival_data() -> String {
  let data = read_data_file("lunar_festival_data.txt");
  let mut day_entries = Vec::new();
  let mut term_entries = Vec::new();
  let mut eve_entries = Vec::new();

  for entry in data.split('@').skip(1) {
    let index = entry[..2].parse::<usize>().unwrap();
    let festival_type = &entry[2..3];
    let value = &entry[3..];

    match festival_type {
      "0" => {
        let month = value[..2].parse::<usize>().unwrap();
        let day = value[2..4].parse::<usize>().unwrap();
        day_entries.push((index, month, day));
      }
      "1" => {
        let term_index = value.parse::<usize>().unwrap();
        term_entries.push((index, term_index));
      }
      "2" => {
        eve_entries.push(index);
      }
      _ => panic!("Unknown festival type: {}", festival_type),
    }
  }

  let mut code = String::new();
  code.push_str("/// 农历节日数据（由build.rs生成）\n");

  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static LUNAR_FESTIVAL_DAY_ENTRIES: [(usize, usize, usize); ");
  code.push_str(&day_entries.len().to_string());
  code.push_str("] = [\n");
  for (index, month, day) in day_entries {
    code.push_str(&format!("    ({}, {}, {}),\n", index, month, day));
  }
  code.push_str("];\n");

  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static LUNAR_FESTIVAL_TERM_ENTRIES: [(usize, usize); ");
  code.push_str(&term_entries.len().to_string());
  code.push_str("] = [\n");
  for (index, term_index) in term_entries {
    code.push_str(&format!("    ({}, {}),\n", index, term_index));
  }
  code.push_str("];\n");

  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static LUNAR_FESTIVAL_EVE_INDICES: [usize; ");
  code.push_str(&eve_entries.len().to_string());
  code.push_str("] = [\n");
  for index in eve_entries {
    code.push_str(&format!("    {},\n", index));
  }
  code.push_str("];\n");

  code
}

fn generate_legal_holiday_data() -> String {
  let data = read_data_file("legal_holiday_data.txt");

  let mut entries = Vec::new();
  let mut i = 0;

  while i < data.len() {
    if i + 12 > data.len() {
      break;
    } // 需要13位，但索引从0开始，所以检查i+12

    let date_part = &data[i..i + 8]; // 年月日
    let year = date_part[0..4].parse::<isize>().unwrap();
    let month = date_part[4..6].parse::<usize>().unwrap();
    let day = date_part[6..8].parse::<usize>().unwrap();
    i += 8;

    let work_part = &data[i..i + 1];
    let work = work_part == "0";
    i += 1;

    let festival_type_part = &data[i..i + 1]; // 节日类型
    let festival_type = festival_type_part.parse::<usize>().unwrap();
    i += 1;

    let sign_char = data.chars().nth(i).unwrap();
    if sign_char != '+' && sign_char != '-' {
      break;
    }
    let sign = if sign_char == '+' { 1 } else { -1 };
    i += 1;

    let offset_str = &data[i..i + 2];
    let offset_value = offset_str.parse::<isize>().unwrap();
    let offset_days = sign * offset_value;
    i += 2;

    entries.push((year, month, day, work, festival_type, offset_days));
  }

  let mut code = String::new();
  code.push_str("/// 法定假日数据（由build.rs生成）\n");
  code.push_str("#[rustfmt::skip]\n");
  code.push_str("pub static LEGAL_HOLIDAY_ENTRIES: [(isize, usize, usize, bool, usize, isize); ");
  code.push_str(&entries.len().to_string());
  code.push_str("] = [\n");

  for (year, month, day, work, festival_type, offset_days) in entries {
    code.push_str(&format!(
      "    ({}, {}, {}, {}, {}, {}),\n",
      year, month, day, work, festival_type, offset_days
    ));
  }

  code.push_str("];\n");
  code
}

fn generate_data_code() -> String {
  let qb = decode(&read_data_file("qb.txt"));
  let sb = decode(&read_data_file("sb.txt"));

  let leap_month_data = generate_leap_month_data();
  let rab_byung_month_days_data = generate_rab_byung_month_days_data();
  let solar_festival_data = generate_solar_festival_data();
  let lunar_festival_data = generate_lunar_festival_data();
  let legal_holiday_data = generate_legal_holiday_data();
  let day_gods_data = generate_day_gods_data();
  let xl1_data = generate_xl1_data();

  format!(
    r#"// 以下代码由 build.rs 自动生成，请勿手动修改
// 本文件位于: $OUT_DIR/util_data.rs

{}

/// QB 数据：节气修正字符串（字节形式）
pub static QB: &[u8] = b"{}";

/// SB 数据：朔望修正字符串（字节形式）
pub static SB: &[u8] = b"{}";

{}

{}

{}

{}

{}

{}

"#,
    xl1_data,
    qb,
    sb,
    leap_month_data,
    rab_byung_month_days_data,
    solar_festival_data,
    lunar_festival_data,
    legal_holiday_data,
    day_gods_data
  )
}
