#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#![allow(dead_code)]

extern crate libc;

use std::os::raw::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;

type TokenizeMode = c_int;
const DefaultMode: TokenizeMode = 0;
const SearchMode: TokenizeMode = 1;

# [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] pub struct Word { pub offset : usize , pub len : usize , }

# [ repr ( C ) ] # [ derive ( Debug , Copy , Clone ) ] 
pub struct CWordWeight { pub word : * mut :: std :: os :: raw :: c_char , pub weight : f64 , }



extern "C" {
    pub fn NewJieba(
        dict_path: *const c_char,
        hmm_path: *const c_char,
        user_dict: *const c_char,
        idf_path: *const c_char,
        stop_words_path: *const c_char,
    ) -> *mut c_void;

    fn FreeJieba(handle: *mut c_void);

    fn FreeWords(words: *mut *mut c_char);

    fn Cut(handle: *mut c_void, sentence: *const c_char, len: c_int) -> *mut *mut c_char;

    fn CutAll (handle : *mut c_void, sentence: * const c_char) -> * mut * mut c_char ; 

    fn CutForSearch(handle : *mut c_void, sentence: * const c_char , is_hmm_used : c_int) -> * mut * mut c_char ;

    fn Tag( handle : *mut c_void , sentence: * const c_char) -> * mut * mut c_char ; 

    fn AddWord(handle : *mut c_void , word: * const c_char) ;

    fn Tokenize(x : *mut c_void, sentence: * const c_char, mode: TokenizeMode , is_hmm_used: c_int) -> * mut Word ;
    
    fn Extract(handle : *mut c_void, sentence: *const c_char, top_k: c_int) -> *mut *mut c_char ;

    fn ExtractWithWeight(handle: *mut c_void, sentence: *const c_char, top_k: c_int) -> *mut CWordWeight; 

    fn FreeWordWeights(wws : *mut CWordWeight);
}

use std::ffi::CString;

#[derive(Debug)]
pub struct Jieba {
    ptr: *mut c_void,
}

impl Jieba {
    pub fn init() -> Self {
        const DICT_PATH: &'static str = "cjieba/dict/jieba.dict.utf8";
        const HMM_PATH: &'static str = "cjieba/dict/hmm_model.utf8";
        const USER_PATH: &'static str = "cjieba/dict/user.dict.utf8";
        const IDF_PATH: &'static str = "cjieba/dict/idf.utf8";
        const STOP_WORDS_PATH: &'static str = "cjieba/dict/stop_words.utf8";

        let dict_path = CString::new(DICT_PATH).unwrap();
        let hmm_path = CString::new(HMM_PATH).unwrap();
        let user_path = CString::new(USER_PATH).unwrap();
        let idf_path = CString::new(IDF_PATH).unwrap();
        let stop_path = CString::new(STOP_WORDS_PATH).unwrap();
        unsafe {
            Jieba {
                ptr: NewJieba(
                    dict_path.as_ptr(),
                    hmm_path.as_ptr(),
                    user_path.as_ptr(),
                    idf_path.as_ptr(),
                    stop_path.as_ptr(),
                ),
            }
        }
    }
    pub fn cut(&self, msg: &str, hmm: bool) -> Vec<String> {
        let hmm = if hmm { 1 } else { 0 };

        let cstr = CString::new(msg).unwrap();
        let cstr_ptr = cstr.as_ptr();
        let words_ptr = unsafe { Cut(self.ptr, cstr_ptr, hmm) };

        let mut cs: Vec<String> = Vec::new();
        let mut idx = 0;

        unsafe {
            while !words_ptr.offset(idx).is_null() && !(*(words_ptr.offset(idx))).is_null() {

                cs.push(
                    CString::from_raw(*(words_ptr.offset(idx))).into_string().unwrap()
                );

                idx += 1;
            }

            FreeWords(words_ptr);
        }

        cs
    }
}

impl Drop for Jieba {
    fn drop(&mut self) {
        unsafe {FreeJieba(self.ptr)};
    }
}