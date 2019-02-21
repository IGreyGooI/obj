#![feature(test)]
#![warn(rust_2018_idioms, rust_2018_compatibility)]
//   Copyright 2017 GFX Developers
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

extern crate obj;
extern crate test;

use obj::{Obj, SimplePolygon};
use std::path::Path;
use obj::IndexTuple;
use std::path;
use std::fs;
use std::io::Read;
use test::Bencher;

#[test]
fn load_test_file() {
    let mut sponza = Obj::<SimplePolygon>::load_file(&Path::new("test_assets/sponza.obj"))
        .unwrap();
    let _ = sponza.load_mtls();
}

#[test]
fn load_test_String() {
    let mut sponza_file = load_file_as_u8(&Path::new("test_assets/sponza.obj"));
    let sponza: Obj<Vec<IndexTuple>> = Obj::load(sponza_file.to_vec().into_boxed_slice()).unwrap();
}

pub fn load_file_as_u8<P: AsRef<path::Path>>(file_path: &P) -> Box<[u8]> {
    let mut buf = Vec::new();
    fs::File::open(file_path)
        .unwrap()
        .read_to_end(&mut buf)
        .unwrap();
    buf.into_boxed_slice()
}

#[bench]
fn bench_load_test_file(b: &mut Bencher) {
    b.iter(|| {
        let mut sponza = Obj::<SimplePolygon>::load_file(&Path::new("test_assets/sponza.obj"))
            .unwrap();
        let _ = sponza.load_mtls();
    });
}

#[bench]
fn bench_load_test_String(b: &mut Bencher) {
    let mut sponza_file = load_file_as_u8(&Path::new("test_assets/sponza.obj"));
    b.iter(|| {
        let sponza: Obj<Vec<IndexTuple>> = Obj::load(sponza_file.to_vec().into_boxed_slice()).unwrap();
    });
}

