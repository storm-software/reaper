#![allow(clippy::too_many_arguments)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(internal_features)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_long_first_doc_paragraph)]

#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
#[rustfmt::skip]
pub mod models;
