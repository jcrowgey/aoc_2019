use std::io::{BufRead, Read};

fn count_byte(v: &Vec<u8>, b: u8) -> usize {
    v.iter().filter(|&x| *x == b).count()
}

fn image_check<I>(mut buf: I, width: usize, height: usize) -> usize
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

        let zero_count = count_byte(&layer, '0' as u8);
        if zero_count < best_zeros {
            best_layer = layer.clone();
            best_zeros = zero_count;
        }
    }

    let one_count = count_byte(&best_layer, '1' as u8);
    let two_count = count_byte(&best_layer, '2' as u8);
    one_count * two_count
}

fn render_image<I>(mut buf: I, width: usize, height: usize) -> Vec<u8>
where
    I: Read,
{


    let mut layer_buf = vec![0u8; width * height];
    let mut final_image = vec!['2' as u8; width * height];
    loop {
        match buf.read_exact(&mut layer_buf) {
            Ok(_) => {},
            Err(_) => {
                break;
            },
        }
        for (i, b) in layer_buf.iter().enumerate() {
            if final_image[i] == ('2' as u8) {  // not yet opaque
                final_image[i] = *b;
            }
        }
        if count_byte(&final_image, '2' as u8) == 0 {
            // nothing left to do, all futher info is invisible
            break;
        }
    }

    final_image
}

pub fn eight_a<I>(buf: I) -> usize
where
    I: Read,
{
    image_check(buf, 25, 6)
}

pub fn eight_b<I>(buf: I) -> Vec<u8>
where
    I: BufRead,
{
    let width = 25;
    let height = 6;
    let image = render_image(buf, width, height);
    for (i, b) in image.iter().enumerate() {
        if i % width == 0 {
            println!();
        }
        if (*b as char) == '0' {
            print!(" ");
        } else if (*b as char) == '1' {
            print!("▒");
        } else {
            print!(" ");
        }
    }
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eight_a() {
        let test_image = b"123456789012\n";
        assert_eq!(image_check(&test_image[..], 3, 2), 1);
    }

    #[test]
    fn test_eight_b() {
        let test_image = b"0222112222120000\n";
        assert_eq!(
            render_image(&test_image[..], 2, 2),
            ['0' as u8, '1' as u8, '1' as u8, '0' as u8].to_vec(),
        );
    }

}
