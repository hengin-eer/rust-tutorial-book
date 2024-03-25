use rand::Rng;

const MAP_N: usize = 25; // 迷路の幅を指定

fn main() {
    let mut rng = rand::thread_rng(); // 乱数生成器のインスタンス化
    let mut maze = [[0; MAP_N]; MAP_N]; // 迷路の初期化

    // 外周を壁化
    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_N - 1] = 1;
        maze[MAP_N - 1][n] = 1;
    }
    // 2マスに1つ壁を配置
    for y in 2..MAP_N - 2 {
        for x in 2..MAP_N - 2 {
            if x % 2 == 1 || y % 2 == 1 { continue; }
            maze[y][x] = 1; // 壁にする

            // 上下左右のいずれかを壁にする
            let rand_num = rng.gen_range(0..=3);
            match rand_num {
                0 => maze[y-1][x] = 1, // 上
                1 => maze[y][x+1] = 1, // 右
                2 => maze[y+1][x] = 1, // 下
                3 => maze[y][x-1] = 1, // 左
                _ => {},
            }
        }
    }
    // 迷路を画面に出力
    let tiles = ["  ", "ZZ"];
    for y in 0..MAP_N {
        for x in 0.. MAP_N {
            print!("{}", tiles[ maze[y][x] ]); // 0,1ごとにタイルの塗りつぶし
        }
        print!("\n");
    }
}