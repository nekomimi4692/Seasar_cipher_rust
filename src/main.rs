use std::{ io::{ stdout, Write, self}  };

fn main() {
    print!("変換したい文字を入力(英文字のみ): ");
    stdout().flush().unwrap();
    let txt = str_input(); 

    print!("次にずらす数値の入力(1~26): ");
    stdout().flush().unwrap();
    let mut num_1 = String::new();
    io::stdin().read_line(&mut num_1).unwrap();

    let mut num_2: i16 = 0;

    match Num::new(num_1) { //ずらす数値の判定
        Ok(num) => {
            num_2 = num.0;
        }
        Err(err_msg) => {
            println!("{}", err_msg);
        }
    };

    let ans = caesar(txt, num_2);

    println!("{}", ans);
}

//inputの関数
fn str_input() -> String {
    let mut word = String::new();
    io::stdin().read_line(&mut word).ok();
    return word.trim().to_string();
}

//ここから下は教えてもらったり、調べてコードをもらったもの

fn caesar(text: String, num: i16) -> String {
    let mut result = String::new();
    // 1文字ずつ繰り返す
    for ch in text.chars() {
        // 小文字は大文字に変換
        let ch = if ch.is_lowercase() { ch.to_ascii_uppercase() } else { ch };
        // 大文字のときのシフト処理
        if ('A'..='Z').contains(&ch) {
            let a = 'A' as i16;
            let enc = (((ch as i16) - a + num + 26) % 26 + a) as u8;
            result.push(enc as char);
        }
        else { // その他はそのまま文字を追加
            result.push(ch);
        }
    }
    result
}

//数値判定の構造体
struct Num(i16);

impl Num {
    fn new(s: String) -> Result<Self, String> {
        if let Ok(num) = s.trim().parse::<i16>() {
            if (1..=26).contains(&num) {
                Ok(Num(num))
            } else {
                Err(format!("{}は1から26の間の数字ではありません。", num))
            }
        } else {
            Err("数字を入力してください".to_string())
        }
    }
}