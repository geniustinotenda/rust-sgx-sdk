// Copyright (c) 2017 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! This crate provides sgx_serialize's two derive macros.
//!
//! ```rust,ignore
//! extern crate sgx_tstd as std; // Must do that!
//! #[derive(Serializable, DeSerializable)]
//! ```
//!

#![crate_name = "sgx_serialize_derive"]
#![crate_type = "rlib"]

// The `quote!` macro requires deep recursion.
#![recursion_limit = "192"]
#![allow(unused_macros)]
#![allow(dead_code)]

#[macro_use]
extern crate quote;

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;
extern crate sgx_serialize_derive_internals as internals;

mod param;
mod bound;
#[macro_use]
mod fragment;

mod encode;
mod decode;

/// `derive_serialize` provides the `Serializable` macro for `sgx_serialize
///
/// `derive_serialize` takes one parameter typed `TokenStream` and parse the
/// input stream brought by it. Then expand the parsed the result as return
/// value.
#[proc_macro_derive(Serializable, attributes(sgx_serialize))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {

    let input = syn::parse_derive_input(&input.to_string()).unwrap();
    match encode::expand_derive_serialize(&input) {
        Ok(expanded) => expanded.parse().unwrap(),
        Err(msg) => panic!(msg),
    }
}


/// `derive_deserialize` provides the `DeSerializable` macro for `sgx_serialize
///
/// `derive_deserialize` takes one parameter typed `TokenStream` and parse the
/// input stream brought by it. Then expand the parsed the result as return
/// value.
#[proc_macro_derive(DeSerializable, attributes(sgx_serialize))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {

    let input = syn::parse_derive_input(&input.to_string()).unwrap();
    match decode::expand_derive_deserialize(&input) {
        Ok(expanded) => expanded.parse().unwrap(),
        Err(msg) => panic!(msg),
    }
}
