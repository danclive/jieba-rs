extern crate gcc;

fn main() {
    let mut c = gcc::Build::new();

    c.warnings(false);

    c.include("cjieba/deps/");

    let files = &[
        "cjieba/lib/jieba.cpp"
    ];

    for file in files.iter() {
        c.file(file);
    }

    c.cpp(true);

    c.compile("jieba");
}
