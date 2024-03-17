fn main() {
    // 符号あり
    println!("i8={}~{}", i8::MIN, i8::MAX);
    println!("i16={}~{}", i16::MIN, i16::MAX);
    println!("i32={}~{}", i32::MIN, i32::MAX);
    println!("");
    // 符号なし
    println!("u8={}~{}", u8::MIN, u8::MAX);
    println!("u16={}~{}", u16::MIN, u16::MAX);
    println!("u32={}~{}", u32::MIN, u32::MAX);
    println!("");
    // 環境ごとに変わる整数型
    println!("isize={}~{}", isize::MIN, isize::MAX);
    println!("usize={}~{}", usize::MIN, usize::MAX);
    println!("usize=u{}", usize::BITS);
}