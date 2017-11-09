// #![allow(non_upper_case_globals)]
// #![allow(non_camel_case_types)]
// #![allow(non_snake_case)]

// extern crate libc;

// use std::os::raw::c_void;
// use std::os::raw::c_char;
// use std::os::raw::c_int;


// pub type Jieba = *mut c_void;

// extern "C" {
//     pub fn NewJieba(
//         dict_path: *const c_char,
//         hmm_path: *const c_char,
//         user_dict: *const c_char,
//         idf_path: *const c_char,
//         stop_words_path: *const c_char,
//     ) -> Jieba;
// }

// extern "C" {
//     pub fn FreeJieba(handle: Jieba);
// }

// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct CJiebaWord {
//     pub word: *const c_char,
//     pub len: usize,
// }
// extern "C" {
//     fn c_word_to_cstring(word: *const c_char, str: *mut c_char) -> c_int;
// }

// unsafe fn c_jieba_word<'a>( ptr: *mut CJiebaWord) ->Vec< &'a CStr> {
// let mut cs =Vec::default();
// let mut idx = 0;

// while !ptr.offset(idx).is_null () {
//     idx +=1;

//     let tmp = ptr.offset(idx);
//     let (w0, len) = ((*tmp).word, (*tmp).len);
//     println!("{}", len);

//     let mut bytes = Vec::default();
//     bytes.push(*w0.offset(-2) as u8);
//     bytes.push(*w0.offset(-1) as u8);
//     bytes.push(*w0.offset(0) as u8);
//     bytes.push(*w0.offset(1) as u8);
//     bytes.push(*w0.offset(3) as u8);
//     bytes.push(*w0.offset(4) as u8);
//     bytes.push(*w0.offset(5) as u8);
//     bytes.push(*w0.offset(6) as u8);
//     println!("{:?}", String::from_utf8_lossy(bytes.as_slice()));
//    cs.push( CStr::from_ptr(w0));
// }
// cs
// }
// extern "C" {
//     pub fn Cut(handle: Jieba, sentence: *const c_char, len: c_int) -> *mut CJiebaWord;
// }

// extern "C" {
//     pub fn CutWithoutTagName(
//         handle: Jieba,
//         sentence: *const c_char,
//         len: c_int,
//         tag_name: *const c_char,
//     ) -> *mut CJiebaWord;
// }

// extern "C" {
//     pub fn FreeWords(words: *mut CJiebaWord);
// }

// extern "C" {
//     pub fn JiebaInsertUserWord(handle: Jieba, word: *const c_char) -> bool;
// }

// pub type Extractor = *mut c_void;

// extern "C" {
//     pub fn NewExtractor(
//         hmm_path: *const c_char,
//         idf_path: *const c_char,
//         stop_word_path: *const c_char,
//         user_dict_path: *const c_char,
//     ) -> Extractor;
// }

// extern "C" {
//     pub fn Extract(
//         handle: Extractor,
//         sentence: *const c_char,
//         len: c_int,
//         topn: c_int,
//     ) -> *mut CJiebaWord;
// }

// extern "C" {
//     pub fn FreeExtractor(handle: Extractor);
// }


// use std::ffi::CString;
// use std::ffi::CStr;

// const DICT_PATH: &'static str = "cjieba/dict/jieba.dict.utf8";
// const HMM_PATH: &'static str = "cjieba/dict/hmm_model.utf8";
// const USER_PATH: &'static str = "cjieba/dict/user.dict.utf8";
// const IDF_PATH: &'static str = "cjieba/dict/idf.utf8";
// const STOP_WORDS_PATH: &'static str = "cjieba/dict/stop_words.utf8";


// fn main() {
//     let dict_path = CString::new(DICT_PATH).unwrap();
//     let hmm_path = CString::new(HMM_PATH).unwrap();
//     let user_path = CString::new(USER_PATH).unwrap();
//     let idf_path = CString::new(IDF_PATH).unwrap();
//     let stop_path = CString::new(STOP_WORDS_PATH).unwrap();

//     let jieba = unsafe {
//         NewJieba(
//             dict_path.as_ptr(),
//             hmm_path.as_ptr(),
//             user_path.as_ptr(),
//             idf_path.as_ptr(),
//             stop_path.as_ptr(),
//         )
//     };

//     unsafe {
//         let arg = "南京市长江大桥";
//         let arg = "hello wolrd";
//         println!("{:?}", arg.len());
//         let word = Cut(jieba, CString::new(arg).unwrap().as_ptr(), 11);

//         let s = CStr::from_ptr((*word).word);

//         // println!("{:?}", s.to_str().unwrap());
//         // println!("{:?}", (*word).len);

//         c_jieba_word(word).into_iter() .for_each(|e| {
//         println!("cstr: {:?}",e);
//         println!("str: {:?}",e.to_str());
//         });
//     }

//     unsafe {
//         FreeJieba(jieba);
//     }
// }

extern crate jieba;
use jieba::*;

fn main() {
    let jb = Jieba::init();
    let ws = jb.cut("他来到了网易杭研大厦", false);
    println!("{:?}", ws);
}