#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

use std::os::raw::c_void;
use std::os::raw::c_char;

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern "C" {
    pub fn FreeWords(words : *mut *mut c_char);
}

pub type Jieba = *mut c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Word {
    pub offset: usize,
    pub len: usize
}

#[test]
fn bindgen_test_layout_Word() {
    assert_eq!(::std::mem::size_of::<Word>(), 16usize, concat!("Size of: ", stringify!(Word)));
    assert_eq!(::std::mem::align_of::<Word>(), 8usize , concat!( "Alignment of ", stringify!(Word)));
    assert_eq!(unsafe {&(*( 0 as * const Word)).offset as * const _ as usize}, 0usize , concat ! ( "Alignment of field: " , stringify ! ( Word ) , "::" , stringify ! ( offset ) ) );
    assert_eq!(unsafe {&(*( 0 as * const Word)).len as * const _ as usize}, 8usize , concat ! ( "Alignment of field: " , stringify ! ( Word ) , "::" , stringify ! ( len ) ) );
}

pub const TokenizeMode_DefaultMode : TokenizeMode = 0;
pub const TokenizeMode_SearchMode : TokenizeMode = 1;
pub type TokenizeMode = ::std::os::raw::c_uint;

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
    pub fn FreeJieba ( arg1 : Jieba , ) ; 
}
extern "C" {
    pub fn Cut ( handle : Jieba , sentence : * const :: std :: os :: raw :: c_char , is_hmm_used : :: std :: os :: raw :: c_int , ) -> * mut * mut :: std :: os :: raw :: c_char ; 
}
extern "C" {
    pub fn CutAll ( handle : Jieba , sentence : * const :: std :: os :: raw :: c_char , ) -> * mut * mut :: std :: os :: raw :: c_char ; 
}
extern "C" {
    pub fn CutForSearch ( handle : Jieba , sentence : * const :: std :: os :: raw :: c_char , is_hmm_used : :: std :: os :: raw :: c_int , ) -> * mut * mut :: std :: os :: raw :: c_char ; 
}
extern "C" {
    pub fn Tag ( handle : Jieba , sentence : * const :: std :: os :: raw :: c_char , ) -> * mut * mut :: std :: os :: raw :: c_char ; 
}
extern "C" {
    pub fn AddWord ( handle : Jieba , word : * const :: std :: os :: raw :: c_char , ) ; 
}
extern "C" {
    pub fn Tokenize ( x : Jieba , sentence : * const :: std :: os :: raw :: c_char , mode : TokenizeMode , is_hmm_used : :: std :: os :: raw :: c_int , ) -> * mut Word ; 
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CWordWeight{
    pub word : * mut :: std :: os :: raw :: c_char ,
    pub weight : f64
}
#[test]
fn bindgen_test_layout_CWordWeight() {
    assert_eq ! ( :: std :: mem :: size_of :: < CWordWeight > ( ) , 16usize , concat ! ( "Size of: " , stringify ! ( CWordWeight ) ) );
    assert_eq ! ( :: std :: mem :: align_of :: < CWordWeight > ( ) , 8usize , concat ! ( "Alignment of " , stringify ! ( CWordWeight ) ) );
    assert_eq ! ( unsafe { & ( * ( 0 as * const CWordWeight ) ) . word as * const _ as usize } , 0usize , concat ! ( "Alignment of field: " , stringify ! ( CWordWeight ) , "::" , stringify ! ( word ) ) );
    assert_eq ! ( unsafe { & ( * ( 0 as * const CWordWeight ) ) . weight as * const _ as usize } , 8usize , concat ! ( "Alignment of field: " , stringify ! ( CWordWeight ) , "::" , stringify ! ( weight ) ) ) ; 
}
extern "C" {
    pub fn Extract ( handle : Jieba , sentence : * const :: std :: os :: raw :: c_char , top_k : :: std :: os :: raw :: c_int , ) -> * mut * mut :: std :: os :: raw :: c_char ; 
}
extern "C" {
    pub fn ExtractWithWeight( handle : Jieba , sentence : * const :: std :: os :: raw :: c_char , top_k : :: std :: os :: raw :: c_int) -> * mut CWordWeight ; 
}
extern "C" {
    pub fn FreeWordWeights( wws : * mut CWordWeight) ; 
}

fn main() {
    println!("{:?}", 123);
}

