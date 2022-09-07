use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // 符号なしの32bit = u32
    // randを用いて1~100までの乱数生成
    let secret_num = rand::thread_rng().gen_range(1..101);

    // loop開始
    loop {
        println!("Please input your guess.");

        // mutable(可変)な文字列型の空変数を生成
        let mut guess = String::new();

        // 標準入力から値を受け取り、参照したguess変数に挿入
        io::stdin()
            .read_line(&mut guess)
            // エラー処理
            .expect("Failed to read line"); // 行の読み込みに失敗しました

        // 文字列型で作成した変数guessを覆い隠す。
        // 数字だった場合はその数字を、文字列だった場合は再度入力させる
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please type number");
                continue;
            }
        };

        println!("You guessed: {}", guess); // 次のように予想しました: {}

        // match式(評価文)
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"), // guess < secret_num
            Ordering::Greater => println!("Too big!"), // guess > secret_num
            Ordering::Equal => {
                println!("You win!"); // guess = secret_num
                break; // loop終了
            }
        }
    }
}
