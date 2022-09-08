// User 構造体
struct User {
    name: String,
    age: u32,
    active: bool,
}

// 引数を取ってUserインスタンスを初期化する関数
fn build_user(name: String) -> User {
    User {
        name,
        active: true,
        age: 80,
    }
}

fn main() {
    let user1 = build_user("hoge".to_string());

    println!("My name is {}", user1.name);
}
