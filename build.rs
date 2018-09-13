extern crate gcc;

fn main() {
    let mut c = gcc::Build::new();

    c.warnings(false);
    c.include("cjieba/deps/");
    c.file("cjieba/lib/jieba.cpp");
    c.cpp(true);
    c.compile("jieba");
}
