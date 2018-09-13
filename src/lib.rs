#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::os::raw::c_void;
use std::os::raw::c_char;
use std::os::raw::c_int;

type TokenizeMode = c_int;
const DefaultMode: TokenizeMode = 0;
const SearchMode: TokenizeMode = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Word {
    pub offset: usize,
    pub len : usize
}

#[repr(C)]
#[derive(Debug, Copy, Clone)] 
pub struct CWordWeight {
    pub word: *mut c_char,
    pub weight: f64
}

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

    fn Cut(handle: *mut c_void, sentence: *const c_char, is_hmm_used: c_int) -> *mut *mut c_char;

    fn CutAll(handle: *mut c_void, sentence: *const c_char) -> *mut *mut c_char; 

    fn CutForSearch(handle: *mut c_void, sentence: *const c_char, is_hmm_used: c_int) -> *mut *mut c_char;

    fn Tag(handle: *mut c_void, sentence: *const c_char) -> *mut *mut c_char;

    fn AddWord(handle: *mut c_void, word: *const c_char);

    fn Tokenize(x: *mut c_void, sentence: *const c_char, mode: TokenizeMode , is_hmm_used: c_int) -> *mut Word;
    
    fn Extract(handle: *mut c_void, sentence: *const c_char, top_k: c_int) -> *mut *mut c_char;

    fn ExtractWithWeight(handle: *mut c_void, sentence: *const c_char, top_k: c_int) -> *mut CWordWeight;

    fn FreeWordWeights(wws: *mut CWordWeight);
}

use std::ffi::CString;

#[derive(Debug)]
pub struct Jieba {
    inner: *mut c_void,
}

pub struct JiebaDict {
    dict: CString,
    hmm: CString,
    user: CString,
    idf: CString,
    stop: CString
}

impl JiebaDict {
    pub fn new(dict: &str, hmm: &str, user_dict: &str, idf_path: &str, stop_path: &str) -> Self {
        JiebaDict {
            dict: CString::new(dict).expect("Can not parser dict path!"),
            hmm: CString::new(hmm).expect("Can not parser hmm model path!"),
            user: CString::new(user_dict).expect("Can not parser user dict path!"),
            idf: CString::new(idf_path).expect("Can not parser idf path!"),
            stop: CString::new(stop_path).expect("Can not parser stop words path!")
        }
    }
}

impl Jieba {
    pub fn with_dict(dict: JiebaDict) -> Self {
        unsafe {
            Jieba {
                inner: NewJieba(
                    dict.dict.as_ptr(),
                    dict.hmm.as_ptr(),
                    dict.user.as_ptr(),
                    dict.idf.as_ptr(),
                    dict.stop.as_ptr()
                )
            }
        }
    }

    pub fn cut(&self, sentence: &str, is_hmm_used: bool) -> Vec<String> {

        let cstr = CString::new(sentence).unwrap();
        let sentence_ptr = cstr.as_ptr();

        let words_ptr = unsafe {
            Cut(self.inner, sentence_ptr, is_hmm_used as c_int)
        };

        convert_ptr_vec(words_ptr)
    }

    pub fn cut_all(&self, sentence: &str) -> Vec<String> {
        let cstr = CString::new(sentence).unwrap();
        let sentence_ptr = cstr.as_ptr();

        let words_ptr = unsafe {
            CutAll(self.inner, sentence_ptr)
        };

        convert_ptr_vec(words_ptr)
    }

    pub fn cut_for_search(&self, sentence: &str, is_hmm_used: bool) -> Vec<String> {
        let cstr = CString::new(sentence).unwrap();
        let sentence_ptr = cstr.as_ptr();

        let words_ptr = unsafe {
            CutForSearch(self.inner, sentence_ptr, is_hmm_used as c_int)
        };

        convert_ptr_vec(words_ptr)
    }

    pub fn tag(&self, sentence: &str) -> Vec<String> {
        let cstr = CString::new(sentence).unwrap();
        let sentence_ptr = cstr.as_ptr();

        let words_ptr = unsafe {
            Tag(self.inner, sentence_ptr)
        };

        convert_ptr_vec(words_ptr)
    }

    pub fn add_word(&self, word: &str) {
        let cstr = CString::new(word).unwrap();
        let word_ptr = cstr.as_ptr();

        unsafe {
            AddWord(self.inner, word_ptr);
        }
    }
}

impl Drop for Jieba {
    fn drop(&mut self) {
        unsafe {
            FreeJieba(self.inner);
        }
    }
}

fn convert_ptr_vec(words_ptr: *mut *mut c_char) -> Vec<String> {
    let mut vec_str = Vec::new();

    unsafe {
        let mut idx = 0;

        while !words_ptr.offset(idx).is_null() && !(*(words_ptr.offset(idx))).is_null() {
            vec_str.push(
                CString::from_raw(*(words_ptr.offset(idx))).into_string().unwrap()
            );

            idx += 1;
        }

        FreeWords(words_ptr);
    }

    vec_str
}
