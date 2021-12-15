pub fn bin_vec_to_dec(vec: &Vec<u8>) -> u32 {
    vec.iter().fold(0, |next, &me| next*2 + me as u32)
}
