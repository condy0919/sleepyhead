use libc;

#[inline]
pub fn epoch_ms_now() -> u64 {
    let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    unsafe {
        libc::clock_gettime(libc::CLOCK_REALTIME_COARSE, &mut ts);
    }

    (ts.tv_sec as u64)
        .wrapping_mul(1000)
        .wrapping_add((ts.tv_nsec as u64).wrapping_div(1000 * 1000))
}

#[inline]
pub fn epoch_ms_diff(diff: i64) -> u64 {
    let now = epoch_ms_now() as i64;
    now.wrapping_add(diff) as u64
}

#[inline]
pub fn epoch_ms_diff_checked(diff: i64) -> Option<u64> {
    let now = epoch_ms_now() as i64;
    now.checked_add(diff).map(|x| x as u64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch_is_always_positive() {
        let now = epoch_ms_now();
        assert!(now > 0);
    }

    #[test]
    fn test_epoch_ms_diff() {
        let now = epoch_ms_diff(i64::max_value());
        assert!((now as i64) < 0);
    }

    #[test]
    fn test_epoch_ms_diff_checked() {
        let now = epoch_ms_diff_checked(i64::max_value());
        assert!(now.is_none());

        let now = epoch_ms_diff_checked(0);
        assert!(now.is_some());
    }
}
