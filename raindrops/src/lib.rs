pub fn raindrops(n: u32) -> String {
    let mut s: String = "".to_string();
    if n % 3 == 0 {
        s += "Pling"
    }
    if n % 5 == 0 {
        s += "Plang"
    }
    if n % 7 == 0 {
        s += "Plong"
    }
    if s.is_empty() {
        s = n.to_string()
    }
    return s;
}
