#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

fn decomp7(value: &u32) -> Vec<u8> {
    let mut value = *value;
    let mut bytes: Vec<u8> = vec![];

    loop {
        let mut byte: u8 = (value % 128) as u8;
        value = (value - byte as u32) / 128;
        if !bytes.is_empty() {
            byte += 128;
        }
        bytes.push(byte);
        if value == 0 {
            break;
        }
    }
    bytes.reverse();
    bytes
}
/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes: Vec<u8> = vec![];

    for value in values {
        let mut decomp = decomp7(value);
        bytes.append(&mut decomp);
    }
    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut nums: Vec<u32> = vec![];
    let mut num: u32 = 0;
    let mut flag: u32 = 0;
    for byte in bytes {
        flag = ((*byte & 128) / 128) as u32;
        let real_byte = (*byte as u32) - 128 * flag;
        let num2 = (num as u64) * 128 + (real_byte as u64);
        if num2 >= 1 << 32 {
            return Err(Error::Overflow);
        }
        num = num2 as u32;
        if flag == 0 {
            nums.push(num);
            num = 0;
        }
    }
    if flag == 0 {
        Ok(nums)
    } else {
        Err(Error::IncompleteNumber)
    }
}
