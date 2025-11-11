use std::fmt::Display;
use std::str::FromStr;
use num_traits::{NumCast, PrimInt};
use self::super::range::Range;

pub type IntRanges<T> = Vec<Range<T>>;

#[derive(Debug, Clone)]
pub struct IntRangesError(&'static str);

const ERR_INT_RANGES: IntRangesError = IntRangesError("intRanges error");


pub fn new_int_ranges_from_list<T, F>(
    list: &[&str],
    parse_fn: F,
) -> Result<IntRanges<T>, IntRangesError>
where
    T: PrimInt,
    F: Fn(&str) -> Result<T, IntRangesError>,
{
    let mut ranges: IntRanges<T> = Vec::new();

    for s in list {
        if s.is_empty() {
            continue;
        }
        let status: Vec<&str> = s.split('-').collect();
        if status.len() > 2 {
            return Err(ERR_INT_RANGES.clone());
        }

        let start = parse_fn(status[0].trim_matches(&['[', ' ', ']'][..]))?;
        match status.len() {
            1 => {
                // 单点
                ranges.push(Range::new(start, start));
            }
            2 => {
                // 区间
                let end = parse_fn(status[1].trim_matches(&['[', ' ', ']'][..]))?;
                ranges.push(Range::new(start, end));
            }
            _ => unreachable!(),
        }
    }

    Ok(ranges)
}

pub trait IntRangesExt<T: PrimInt>: Sized {
    fn check(&self, status: T) -> bool;
    fn to_string(&self) -> String; // 避免与 Display 冲突
    fn range<F: FnMut(T) -> bool>(&self, f: F);
    fn merge(self) -> Self;
}

impl<T> IntRangesExt<T> for IntRanges<T>
where
    T: PrimInt + Display,
{
    fn check(&self, status: T) -> bool {
        if self.is_empty() {
            return true;
        }
        self.iter().any(|seg| seg.contains(&status))
    }

    fn to_string(&self) -> String {
        if self.is_empty() {
            return "*".to_string();
        }
        let mut terms = Vec::with_capacity(self.len());
        for r in self {
            if r.start == r.end {
                terms.push(format!("{}", r.start));
            } else {
                terms.push(format!("{}-{}", r.start, r.end));
            }
        }
        terms.join("/")
    }

    fn range<F: FnMut(T) -> bool>(&self, mut f: F) {
        if self.is_empty() {
            return;
        }
        let one = T::one();
        'outer: for r in self {
            let mut i = r.start;
            loop {
                if !f(i) {
                    break 'outer;
                }
                if i == r.end {
                    break;
                }
                match i.checked_add(&one) {
                    Some(next) => {
                        i = next;
                    }
                    None => break, // overflow
                }
            }
        }
    }

    fn merge(mut self) -> Self {
        if self.is_empty() {
            return self;
        }
        self.sort_unstable_by_key(|r| r.start);

        let one = T::one();
        let mut merged: Vec<Range<T>> = Vec::with_capacity(self.len());
        let mut first = true;

        for r in self.into_iter() {
            if first {
                merged.push(r);
                first = false;
                continue;
            }

            let last = merged.last_mut().unwrap();
            let end_plus_one = last.end.checked_add(&one);

            let is_disjoint = match end_plus_one {
                Some(end) => r.start > end, // disjoint
                None => r.start > last.end,        // 溢出时，只能要求严格大于 end
            };

            if is_disjoint {
                merged.push(r);
            } else {
                // 合并：扩展右端
                if r.end > last.end {
                    last.end = r.end;
                }
            }
        }

        merged
    }
}

pub fn parse_unsigned<T>(s: &str) -> Result<T, IntRangesError>
where
    T: PrimInt + NumCast,
{
    let v = u64::from_str(s).map_err(|_| ERR_INT_RANGES.clone())?;
    NumCast::from(v).ok_or_else(|| ERR_INT_RANGES.clone())
}



pub fn new_int_ranges<T, F>(expected: &str, parse_fn: F) -> Result<IntRanges<T>, IntRangesError>
where
    T: PrimInt,
    F: Fn(&str) -> Result<T, IntRangesError>,
{
    let mut s = expected.trim().to_string();
    if s.is_empty() || s == "*" {
        return Ok(Vec::new());
    }

    // 兼容逗号
    s = s.replace(',', "/");
    let parts: Vec<&str> = s.split('/').collect();

    if parts.len() > 28 {
        return Err(IntRangesError("intRanges error, too many ranges to use, maximum support 28 ranges"));
    }

    new_int_ranges_from_list(&parts, parse_fn)
}

pub fn new_unsigned_ranges<T>(expected: &str) -> Result<IntRanges<T>, IntRangesError>
where
    T: PrimInt + NumCast,
{
    new_int_ranges(expected, parse_unsigned::<T>)
}

pub fn new_unsigned_ranges_from_list<T>(list: &[&str]) -> Result<IntRanges<T>, IntRangesError>
where
    T: PrimInt + NumCast,
{
    new_int_ranges_from_list(list, parse_unsigned::<T>)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::range::Range; // 如果 Range 不在 super::super，请按你的路径调整
    use crate::*; // 如无需要可移除
    use std::iter::FromIterator;

    // 便于调用 trait 方法（to_string/check/range/merge）
    use super::IntRangesExt;

    #[test]
    fn parse_unsigned_basic_and_commas() {
        // 支持逗号与斜杠
        let r1: IntRanges<u32> = new_unsigned_ranges("200/204/401-429/501-503").unwrap();
        let r2: IntRanges<u32> = new_unsigned_ranges("200,204,401-429,501-503").unwrap();
        assert_eq!(r1.len(), 4);
        assert_eq!(r1, r2);

        // 单点与区间
        assert!(r1.check(200));
        assert!(r1.check(204));
        assert!(r1.check(401));
        assert!(r1.check(429));
        assert!(r1.check(503));
        assert!(!r1.check(430));
    }


    #[test]
    fn star_means_all() {
        // "*" 或空字符串 => 空 ranges => check 恒为 true
        let a: IntRanges<u16> = new_unsigned_ranges("*").unwrap();
        let b: IntRanges<u16> = new_unsigned_ranges("").unwrap();
        assert!(a.check(0));
        assert!(a.check(65535));
        assert!(b.check(12345));
        assert_eq!(a.to_string(), "*");
        assert_eq!(b.to_string(), "*");
    }

    #[test]
    fn to_string_repr() {
        // 单点 + 区间的混合
        let r: IntRanges<u32> = vec![
            Range::new(200u32, 200),
            Range::new(204u32, 204),
            Range::new(401u32, 429),
            Range::new(501u32, 503),
        ];
        assert_eq!(r.to_string(), "200/204/401-429/501-503");
    }

    #[test]
    fn iterate_range_and_sum() {
        // 迭代 3-5 => 3 + 4 + 5
        let r: IntRanges<u32> = new_unsigned_ranges("3-5").unwrap();
        let mut sum: u64 = 0;
        r.range(|x| { sum += x as u64; true });
        assert_eq!(sum, 12);
    }

    #[test]
    fn iterate_early_stop() {
        // 早停：在 10-20 中遇到 13 返回 false 立即停止
        let r: IntRanges<u32> = new_unsigned_ranges("10-20").unwrap();
        let mut seen = Vec::new();
        r.range(|x| { seen.push(x); x != 13 });
        assert_eq!(seen, vec![10, 11, 12, 13]);
    }

    #[test]
    fn iterate_handles_u8_overflow_safely() {
        // 检查上界附近：u8 的 254-255
        let r: IntRanges<u8> = new_unsigned_ranges("254-255").unwrap();
        let mut v = Vec::new();
        r.range(|x| { v.push(x); true });
        assert_eq!(v, vec![254u8, 255u8]);

        // 只单点 255
        let r2: IntRanges<u8> = new_unsigned_ranges("255").unwrap();
        let mut v2 = Vec::new();
        r2.range(|x| { v2.push(x); true });
        assert_eq!(v2, vec![255u8]);
    }

    #[test]
    fn merge_overlapping_and_adjacent() {
        // 合并相邻与重叠：1-3/4/6-8/9 => 1-4/6-9
        let r: IntRanges<u32> = new_unsigned_ranges("1-3/4/6-8/9").unwrap();
        let merged = r.merge();
        assert_eq!(merged.to_string(), "1-4/6-9");

        // 重叠：10-15/12-20 => 10-20
        let r2: IntRanges<u32> = new_unsigned_ranges("10-15/12-20").unwrap();
        let merged2 = r2.merge();
        assert_eq!(merged2.to_string(), "10-20");
    }

    #[test]
    fn merge_with_u8_boundary_wrap_should_not_cross() {
        // 254-255 与 0-1 不能合并（按值排序后为 0-1 / 254-255）
        let r: IntRanges<u8> = new_unsigned_ranges("254-255/0-1").unwrap().merge();
        assert_eq!(r.to_string(), "0-1/254-255");
    }

    #[test]
    fn new_int_ranges_from_list_basic() {
        // 直接从列表构造（无符号）
        let parts = vec!["200", "204", "401-429", "501-503"];
        let r: IntRanges<u32> = new_int_ranges_from_list(&parts, parse_unsigned::<u32>).unwrap();
        assert!(r.check(401));
        assert!(!r.check(430));
        assert_eq!(r.to_string(), "200/204/401-429/501-503");
    }

    #[test]
    fn too_many_ranges_error() {
        // 构造 29 段，超过 28 上限
        let input = (0..29).map(|i| i.to_string()).collect::<Vec<_>>().join("/");
        let res: Result<IntRanges<u32>, _> = new_unsigned_ranges(&input);
        assert!(res.is_err());
    }

    #[test]
    fn invalid_piece_error() {
        // 非法的 "1-2-3"
        let parts = vec!["1-2-3"];
        let res: Result<IntRanges<u32>, _> =
            new_int_ranges_from_list(&parts, parse_unsigned::<u32>);
        assert!(res.is_err());

        // 非法数字
        let parts2 = vec!["abc"];
        let res2: Result<IntRanges<u32>, _> =
            new_int_ranges_from_list(&parts2, parse_unsigned::<u32>);
        assert!(res2.is_err());
    }

    #[test]
    fn order_normalization_on_range_new() {
        // Range::new 会自动规范化区间端点
        let r = Range::new(10i32, -3i32);
        assert_eq!((r.start, r.end), (-3, 10));
        assert!(r.contains(&0));
        assert!(!r.contains(&11));
    }

    #[test]
    fn empty_ranges_check_true_and_no_iteration() {
        let r: IntRanges<u32> = Vec::new();
        assert!(r.check(123)); // 空 => "*"

        let mut called = false;
        r.range(|_| { called = true; true });
        assert!(!called);
        assert_eq!(r.to_string(), "*");
    }
}
