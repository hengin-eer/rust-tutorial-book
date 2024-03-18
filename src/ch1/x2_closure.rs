fn main() {
    // 値を倍するクロージャ―の定義
    let x2 = |n| n*2;

    println!("{}", x2(2));
    println!("{}", x2(8));
}