extern crate test;
use crate::types::*;

/// Calculate the checksum of some data
pub fn calculate(data: &DataBytes<u8>) -> Checksum {
    let mut checksum = 0u8;

    data.iter().for_each(|x| {
        checksum = (checksum.overflowing_add(*x)).0 % 255;
    });

    0xff_u8.overflowing_sub(checksum).0.overflowing_add(1).0
}

/// Calculate the TEXT Checksum of a frame, ignoring its current checksum.
///
/// The checksum needs to be calculated from the start (0d) till the end of the frame, excluding the checksum value from the frame (d8).
/// The checksum is the complement allowing for the frame checksum to be 0x00.
/// Reference: https://www.victronenergy.com/live/vedirect_protocol:faq#q8how_do_i_calculate_the_text_checksum
pub fn calculate_for_frame(frame: &FrameBytes<u8>) -> Checksum {
    calculate(frame.split_last().unwrap().1)
}

pub fn append(data: &FrameBytes<u8>, checksum: Checksum) -> Vec<u8> {
    [data, &vec![checksum]].concat()
}

/// Verify a frame using its checksum. Since the checksum is calculating as complement to have checksum of the frame equal to 0,
/// we can run the same checksum algorithm and check that the checksum is 0.
pub fn verify(frame: &FrameBytes<u8>) -> bool {
    calculate(frame) == 0
}

#[cfg(test)]
mod tests_checksum {
    use super::*;

    #[test]
    fn test_append_checksum() {
        assert_eq!(append(&vec![1, 2], 3), vec![1, 2, 3]);
    }

    #[test]
    fn test_calculate_text_checksum_for_data() {
        let data = vec![0x0d, 0x0a];
        let checksum = calculate(&data);
        assert_eq!(checksum, 0xe9);
    }

    #[test]
    fn test_calculate_text_checksum_short() {
        let frame = vec![0x0d, 0x0a, 0xd8];
        let checksum = calculate_for_frame(&frame);
        assert_eq!(checksum, 0xe9);
    }

    #[test]
    fn test_calculate_text_checksum_short2() {
        let frame = vec![0x0d, 0x0a, 0x7f, 0x7f, 0xd8];
        let checksum = calculate_for_frame(&frame);
        assert_eq!(checksum, 0xeb);
    }

    #[test]
    fn test_calculate_text_checksum_real() {
        let frame = vec![
            0x0d, 0x0a, 0x50, 0x49, 0x44, 0x09, 0x30, 0x78, 0x32, 0x30, 0x33, 0x0d, 0x0a, 0x56,
            0x09, 0x32, 0x36, 0x32, 0x30, 0x31, 0x0d, 0x0a, 0x49, 0x09, 0x30, 0x0d, 0x0a, 0x50,
            0x09, 0x30, 0x0d, 0x0a, 0x43, 0x45, 0x09, 0x30, 0x0d, 0x0a, 0x53, 0x4f, 0x43, 0x09,
            0x31, 0x30, 0x30, 0x30, 0x0d, 0x0a, 0x54, 0x54, 0x47, 0x09, 0x2d, 0x31, 0x0d, 0x0a,
            0x41, 0x6c, 0x61, 0x72, 0x6d, 0x09, 0x4f, 0x46, 0x46, 0x0d, 0x0a, 0x52, 0x65, 0x6c,
            0x61, 0x79, 0x09, 0x4f, 0x46, 0x46, 0x0d, 0x0a, 0x41, 0x52, 0x09, 0x30, 0x0d, 0x0a,
            0x42, 0x4d, 0x56, 0x09, 0x37, 0x30, 0x30, 0x0d, 0x0a, 0x46, 0x57, 0x09, 0x30, 0x33,
            0x30, 0x37, 0x0d, 0x0a, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x09, 0xd8,
        ];
        let checksum = calculate_for_frame(&frame);
        assert_eq!(checksum, 0xd8);
    }

    #[test]
    fn test_verify_text_checksum_real() {
        let frame = vec![
            0x0d, 0x0a, 0x50, 0x49, 0x44, 0x09, 0x30, 0x78, 0x32, 0x30, 0x33, 0x0d, 0x0a, 0x56,
            0x09, 0x32, 0x36, 0x32, 0x30, 0x31, 0x0d, 0x0a, 0x49, 0x09, 0x30, 0x0d, 0x0a, 0x50,
            0x09, 0x30, 0x0d, 0x0a, 0x43, 0x45, 0x09, 0x30, 0x0d, 0x0a, 0x53, 0x4f, 0x43, 0x09,
            0x31, 0x30, 0x30, 0x30, 0x0d, 0x0a, 0x54, 0x54, 0x47, 0x09, 0x2d, 0x31, 0x0d, 0x0a,
            0x41, 0x6c, 0x61, 0x72, 0x6d, 0x09, 0x4f, 0x46, 0x46, 0x0d, 0x0a, 0x52, 0x65, 0x6c,
            0x61, 0x79, 0x09, 0x4f, 0x46, 0x46, 0x0d, 0x0a, 0x41, 0x52, 0x09, 0x30, 0x0d, 0x0a,
            0x42, 0x4d, 0x56, 0x09, 0x37, 0x30, 0x30, 0x0d, 0x0a, 0x46, 0x57, 0x09, 0x30, 0x33,
            0x30, 0x37, 0x0d, 0x0a, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x09, 0xd8,
        ];
        assert_eq!(verify(&frame), true);
    }

    #[test]
    fn test_verify_text_checksum_real_error() {
        let frame = vec![
            0x0d, 0x0a, 0x50, 0x49, 0x44, 0x09, 0x30, 0x78, 0x32, 0x30, 0x33, 0x0d, 0x0a, 0x56,
            0x09, 0x32, 0x36, 0x32, 0x30, 0x31, 0x0d, 0x0a, 0x49, 0x09, 0x30, 0x0d, 0x0a, 0x50,
            0x09, 0x30, 0x0d, 0x0a, 0x43, 0x45, 0x09, 0x30, 0x0d, 0x0a, 0x53, 0x4f, 0x43, 0x09,
            0x31, 0x30, 0x30, 0x30, 0x0d, 0x0a, 0x54, 0x54, 0x47, 0x09, 0x2d, 0x31, 0x0d, 0x0a,
            0x41, 0x6c, 0x61, 0x72, 0x6d, 0x09, 0x4f, 0x46, 0x46, 0x0d, 0x0a, 0x52, 0x65, 0x6c,
            0x61, 0x79, 0x09, 0x4f, 0x46, 0x46, 0x0d, 0x0a, 0x41, 0x52, 0x09, 0x30, 0x0d, 0x0a,
            0x42, 0x4d, 0x56, 0x09, 0x37, 0x30, 0x30, 0x0d, 0x0a, 0x46, 0x57, 0x09, 0x30, 0x33,
            0x30, 0x37, 0x0d, 0x0a, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x09, 0xff,
        ];
        assert_eq!(verify(&frame), false);
    }

    #[test]
    fn test_calculate_text_checksum_2() {
        let frame = "\r\nfield1\tvalue1\r\nfield2\tvalue2\r\nChecksum\te".as_bytes();

        assert_eq!(calculate_for_frame(&frame), 0x65);
    }

    #[test]
    fn test_verify_text_checksum_2() {
        let frame = "\r\nfield1\tvalue1\r\nfield2\tvalue2\r\nChecksum\te".as_bytes();

        assert_eq!(verify(&frame), true);
    }
}

// TODO: switch to criterion to remain on stable
#[cfg(test)]
mod benchmarkss {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_checksum(b: &mut Bencher) {
        let data = vec![0x0d, 0x0a, 0xf0, 0x0f, 0xff, 0x55];
        let checksum = calculate(&data);
        assert_eq!(checksum, 150);
        b.iter(|| calculate(&data));
    }
}
