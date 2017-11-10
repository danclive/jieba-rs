
extern crate jieba;
use jieba::*;

fn main() {
    let arg = std::env::args().nth(1).unwrap_or(
        "他来到了网易杭研大厦"
            .to_owned(),
    );
    let jb = Jieba::init();

    let ws = jb.cut(&arg, false);
    println!("{:?}", ws);
}