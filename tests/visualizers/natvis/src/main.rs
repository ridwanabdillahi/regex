/***

// See https://github.com/rust-lang/rust-dbg-ext/blob/main/test-framework/dbt/README.md
// for more information on how these test scripts work.

#if @cdb
  // Start running the program
  g

  // The following line checks that we actually hit the breakpoint.
  #check Breakpoint 0 hit

  // Ensure the Natvis file for the regex crate is loaded.
  .nvlist
  #check natvis.exe (embedded NatVis "@{ .* }@natvis-0.natvis")

  // Check the Natvis visualization for re::unicode::Regex
  dv
  dx re
  #check re               : { text="hits="[0-9]+"" } [Type: regex::re_unicode::Regex]
  #check   [<Raw View>]     [Type: regex::re_unicode::Regex]
  #check   [Reference count] : 0x2 [Type: core::sync::atomic::AtomicUsize]
  #check   [Weak reference count] : 0x1 [Type: core::sync::atomic::AtomicUsize]
  #check   [+0xc00] res              : { len=0x1 } [Type: alloc::vec::Vec<alloc::string::String,alloc::alloc::Global>]
  #check   [+0x000] nfa              [Type: regex::prog::Program]
  #check   [+0x320] dfa              [Type: regex::prog::Program]
  #check   [+0x640] dfa_reverse      [Type: regex::prog::Program]
  #check   [+0x960] suffixes         [Type: regex::literal::imp::LiteralSearcher]
  #check   [+0xc18] ac               : None [Type: enum$<core::option::Option<aho_corasick::ahocorasick::AhoCorasick<u32> >, 0, 1, Some>]
  #check   [+0xda0] match_type       : Dfa [Type: enum$<regex::exec::MatchType>]

  dx captures
  #check captures         : { named_groups=0x0 } [Type: regex::re_unicode::Captures]
  #check   [<Raw View>]     [Type: regex::re_unicode::Captures]
  #check   [text]           : "<Type><hits="12123123123"></hits></Type>" [Type: str]
  #check   [named_groups]   : { len=0x0 } [Type: @{ .* }@]
  #check   [0]              : @{ .* }@ : "hits="12123123123"" [Type: char *]

  g

  dx m
  #check m                : "hits="12123123123"" [Type: regex::re_unicode::Match]
  #check   [<Raw View>]     [Type: regex::re_unicode::Match]
  #check   [text]           : "<Type><hits="12123123123"></hits></Type>" [Type: str]
  #check   [match_text]     : "hits="12123123123""
  #check   [start]          : 0x7 [Type: unsigned __int64]
  #check   [end]            : 0x19 [Type: unsigned __int64]

  q

#if not @cdb
  #ignore-test

***/

use regex::{Regex, Match};

pub fn main() {
    let re = Regex::new(r#"hits="[0-9]+""#).unwrap();
    let text = r#"<Type><hits="12123123123"></hits></Type>"#;

    let captures = re.captures(text).unwrap();
    let matches = captures.iter().filter_map(|capture| capture).collect::<Vec<Match>>();
    assert_eq!(1, matches.len());
    _break(); // #break

    for m in matches {
      _break(); // #break
      assert_eq!(r#"hits="12123123123""#, m.as_str());
    }
}

#[inline(never)]
pub fn _break() {}
