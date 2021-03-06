//
// Copyright 2018-2019 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # Hammersbald Blockchain store
//!
//! A very fast persistent blockchain store
//!

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(missing_docs)]
#![deny(unused_must_use)]
#![forbid(unsafe_code)]

extern crate bitcoin_hashes;
extern crate byteorder;
extern crate lru_cache;
extern crate rand;

mod api;
mod asyncfile;
mod cachedfile;
mod datafile;
mod error;
mod format;
mod logfile;
mod memtable;
mod page;
mod pagedfile;
mod persistent;
mod pref;
mod rolledfile;
mod singlefile;
mod stats;
mod tablefile;
mod transient;

pub use api::{persistent, transient, HammersbaldAPI, HammersbaldDataReader, HammersbaldDataWriter, HammersbaldIterator};
pub use error::Error;
pub use pref::PRef;
