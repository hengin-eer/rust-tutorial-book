use rand::seq::SliceRandom; // shuffleメソッドが使えるようになる

fn main() {
    let mut nums = [0; 75]; // 75の配列を初期化
    for i in 1..75 { nums[i-1] = i; } // 1~75をそれぞれ代入

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng); // 配列をシャッフル（シャッフルのためにmut）

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x; // y行目を考慮してx5している
            if i == 12 { // ワイルドカードの時
                print!(" *,");
            } else {
                print!("{:3},", nums[i]); // 3文字分の出力をしてフォーマット揃え
            }
        }
        println!("");
    }
}