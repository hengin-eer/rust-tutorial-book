fn main() {
    let moon_far = 384400.0;
    let car_fast = 80.0;
    let btrain_fast = 300.0;
    println!("月まで車で{}日", moon_far / car_fast / 24.0);
    println!("月まで新幹線で{}日", moon_far / btrain_fast / 24.0);
}