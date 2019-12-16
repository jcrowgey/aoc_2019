use std::io::{BufRead, Read};

pub fn image_check<I>(mut buf: I, width: usize, height: usize) -> usize
where
    I: Read,
{
    let mut layer = vec![0u8; width * height];
    let mut best_layer = vec![0u8; width * height];
    let mut best_zeros: usize  = usize::max_value();

    loop {
        match buf.read_exact(&mut layer) {
            Ok(_) => {},
            Err(_) => {
                break;
            },
        }

        let zero_count = layer.iter().filter(|&n| *n == '0' as u8).count();
        if zero_count < best_zeros {
            best_layer = layer.clone();
            best_zeros = zero_count;
        }
    }

    let one_count = best_layer.iter().filter(|&n| *n == '1' as u8 ).count();
    let two_count = best_layer.iter().filter(|&n| *n == '2' as u8 ).count();
    one_count * two_count
}

pub fn eight_a<I>(buf: I) -> usize
where
    I: Read,
{
    image_check(buf, 25, 6)
}

pub fn eight_b<I>(buf: I) -> i32
where
    I: BufRead,
{
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight_a() {
        let test_image = b"123456789012\n";
        assert_eq!(image_check(&test_image[..], 3, 2), 1);
    }

}
