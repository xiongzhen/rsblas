pub fn check_inc<T>(n: isize, x: &[T], inc: isize) -> bool {
    if n <= 0 {
        return false;
    }

    if inc > 0 {
        x.len() >= ((n as usize) - 1) * (inc as usize) + 1
    } else {
        x.len() >= ((n as usize) - 1) * ((-inc) as usize) + 1
    }
}

pub fn get_first_index(n: usize, inc: isize) -> usize {
    if inc > 0 {
        0
    } else {
        (n - 1) * ((-inc) as usize)
    }
}