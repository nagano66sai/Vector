/*1から100までの整数を14個発生し、それらをベクターに納めます(pushする)。
3と12がそのベクターの何番目にあるか、その位置を出力します。
(existとは"存在する"という意味です。
Secret numberとは3と12のことです。)
*/





extern crate rand;
use rand::Rng;
fn main() {
    let mut c = vec![];

    for _i in 1..15 {
        let s = rand::thread_rng().gen_range(1, 101);
        println!("the number is {}", s);
        c.push(s);
    }
    println!(" c={:?}", c);
    for j in 0..14 {
        let x = c.pop();
        if x == Some(12) || x == Some(3) {
            println!(
                "Secret number is {}...{:?}は{}{}",
                "exist",
                x.unwrap(),
                14 - j,
                "番目にあります。"
            );
        } else {
            println!("Secret number is {}", "no exist");
        }
    }
}
