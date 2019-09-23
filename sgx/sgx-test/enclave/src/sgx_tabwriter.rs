use tabwriter::TabWriter;
use std::prelude::v1::*;
use std::vec::Vec;
use std::io::Write;
fn ordie<T, E: ToString>(r: Result<T, E>) -> T {
    match r {
        Ok(r) => r,
        Err(e) => panic!("{}", e.to_string()),
    }
}

fn readable_str(s: &str) -> String {
    s.replace(" ", "·")
}

fn tabw() -> TabWriter<Vec<u8>> {
    TabWriter::new(Vec::new())
}

fn tabify(mut tw: TabWriter<Vec<u8>>, s: &str) -> String {
    ordie(write!(&mut tw, "{}", s));
    ordie(tw.flush());
    ordie(String::from_utf8(tw.into_inner().unwrap()))
}

fn iseq(tw: TabWriter<Vec<u8>>, s: &str, expected: &str) {
    let written = tabify(tw, s);
    if expected != written {
        panic!("\n\nexpected:\n-----\n{}\n-----\ngot:\n-----\n{}\n-----\n\n",
               readable_str(expected), readable_str(&written));
    }
}


pub fn test_no_cells() {
    iseq(tabw(), "foo\nbar\nfubar", "foo\nbar\nfubar");
}

pub fn test_no_cells_trailing() {
    iseq(tabw(), "foo\nbar\nfubar\n", "foo\nbar\nfubar\n");
}


pub fn test_no_cells_prior() {
    iseq(tabw(), "\nfoo\nbar\nfubar", "\nfoo\nbar\nfubar");
}


pub fn test_empty() {
    iseq(tabw(), "", "");
}


pub fn test_empty_lines() {
    iseq(tabw(), "\n\n\n\n", "\n\n\n\n");
}


pub fn test_empty_cell() {
    iseq(tabw().padding(0).minwidth(2), "\t\n", "  \n");
}


pub fn test_empty_cell_no_min() {
    iseq(tabw().padding(0).minwidth(0), "\t\n", "\n");
}


pub fn test_empty_cells() {
    iseq(tabw().padding(0).minwidth(2), "\t\t\n", "    \n");
}


pub fn test_empty_cells_no_min() {
    iseq(tabw().padding(0).minwidth(0), "\t\t\n", "\n");
}


pub fn test_empty_cells_ignore_trailing() {
    iseq(tabw().padding(0).minwidth(2), "\t\t\t", "    ");
}


pub fn test_one_cell() {
    iseq(tabw().padding(2).minwidth(2), "a\tb\nxx\tyy", "a   b\nxx  yy");
}


pub fn test_no_padding() {
    iseq(tabw().padding(0).minwidth(2), "a\tb\nxx\tyy", "a b\nxxyy");
}


pub fn test_minwidth() {
    iseq(tabw().minwidth(5).padding(0),
         "a\tb\nxx\tyy", "a    b\nxx   yy");
}


pub fn test_contiguous_columns() {
    iseq(tabw().padding(1).minwidth(0),
         "x\tfoo\tx\nx\tfoofoo\tx\n\nx\tfoofoofoo\tx",
         "x foo    x\nx foofoo x\n\nx foofoofoo x");
}


pub fn test_unicode() {
    iseq(tabw().padding(2).minwidth(2),
         "a\tÞykkvibær\tz\naaaa\tïn Bou Chella\tzzzz\na\tBâb el Ahmar\tz",
         "a     Þykkvibær      z\n\
          aaaa  ïn Bou Chella  zzzz\n\
          a     Bâb el Ahmar   z")
}


pub fn test_contiguous_columns_complex() {
    iseq(tabw().padding(1).minwidth(3),
"
fn foobar() {
 	let mut x = 1+1;	// addition
 	x += 1;	// increment in place
 	let y = x * x * x * x;	// multiply!

 	y += 1;	// this is another group
 	y += 2 * 2;	// that is separately aligned
}
",
"
fn foobar() {
    let mut x = 1+1;       // addition
    x += 1;                // increment in place
    let y = x * x * x * x; // multiply!

    y += 1;     // this is another group
    y += 2 * 2; // that is separately aligned
}
");
}

