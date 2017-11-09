#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::os::raw::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;


pub type Jieba = *mut c_void;

extern "C" {
    pub fn NewJieba(
        dict_path: *const c_char,
        hmm_path: *const c_char,
        user_dict: *const c_char,
        idf_path: *const c_char,
        stop_words_path: *const c_char
    ) -> Jieba;
}

extern "C" {
    pub fn FreeJieba(handle: Jieba);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CJiebaWord {
    pub word: *const c_char,
    pub len: usize
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CWordWeight {
    pub word: *mut c_char ,
    pub weight: f64
}

extern "C" {
    pub fn Cut2(handle: Jieba, sentence: *const c_char, len: c_int) -> *mut *mut c_char; 
}

extern "C" {
    pub fn CutWithoutTagName(handle: Jieba, sentence: *const c_char, len: c_int, tag_name: *const c_char) -> *mut CJiebaWord;
}

extern "C" {
    pub fn FreeWords(words : *mut CJiebaWord);
}

extern "C" {
    pub fn JiebaInsertUserWord(handle: Jieba, word: *const c_char) -> bool;
}

pub type Extractor = *mut c_void;

extern "C" {
    pub fn NewExtractor(
        hmm_path: *const c_char,
        idf_path: *const c_char,
        stop_word_path: *const c_char,
        user_dict_path: *const c_char,
    ) -> Extractor;
}

extern "C" {
    pub fn Extract(handle: Extractor, sentence: *const c_char, len: c_int, topn: c_int) -> *mut CJiebaWord;
}

extern "C" {
    pub fn FreeExtractor(handle: Extractor);
}


use std::ffi::CString;
use std::ffi::CStr;

const DICT_PATH: &'static str = "/home/simple/github/jieba-rs/cjieba/dict/jieba.dict.utf8";
const HMM_PATH: &'static str = "/home/simple/github/jieba-rs/cjieba/dict/hmm_model.utf8";
const USER_PATH: &'static str = "/home/simple/github/jieba-rs/cjieba/dict/user.dict.utf8";
const IDF_PATH: &'static str = "/home/simple/github/jieba-rs/cjieba/dict/idf.utf8";
const STOP_WORDS_PATH: &'static str = "/home/simple/github/jieba-rs/cjieba/dict/stop_words.utf8";


fn main() {
    println!("{:?}", "hello world");

    let dict_path = CString::new(DICT_PATH).unwrap();
    let hmm_path = CString::new(HMM_PATH).unwrap();
    let user_path = CString::new(USER_PATH).unwrap();
    let idf_path = CString::new(IDF_PATH).unwrap();
    let stop_path = CString::new(STOP_WORDS_PATH).unwrap();

    let jieba = unsafe {
        NewJieba(
            dict_path.as_ptr(),
            hmm_path.as_ptr(),
            user_path.as_ptr(),
            idf_path.as_ptr(),
            stop_path.as_ptr()
        ) 
    };

    unsafe {
        let word = Cut2(jieba, CString::new("南京市长江大桥我是拖拉机学院手扶拖拉机专业的。不用多久，我就会升职加薪，当上CEO，走上人生巅峰。").unwrap().as_ptr(), 21);

        //let s1: Box<CJiebaWord> = Box::from_raw(word);

        //println!("{:?}", s1);


        // let z: Vec<CJiebaWord> = Vec::from_raw_parts(word, 2, 2);

        // println!("{:?}", z);

        // for i in z {
        //     let s: Vec<u8> = Vec::from_raw_parts(i.word as *mut u8, i.len, i.len);

        //     println!("{:?}", s);

        //     println!("{:?}", String::from_utf8_lossy(&s[1..]));
        // }

        //let s1: Box<CJiebaWord> = Box::from_raw(word);

        let s: Vec<*mut c_char> = Vec::from_raw_parts(word, 2, 2);

        println!("{:?}", CString::from_raw(s[0]).to_str());
        println!("{:?}", CString::from_raw(s[1]).to_str());
    }

    unsafe {
        FreeJieba(jieba);
    }
}

