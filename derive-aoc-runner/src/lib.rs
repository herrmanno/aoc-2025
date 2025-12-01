//! Macros for deriving Advent of Code runner utility
//!
//! # Example
//!
//! ## Using the default [TimeAnalyzer][aoc_runner::TimeAnalyzer]
//! ```rust
//! use aoc_runner::{Day,Analyzer};
//! use derive_aoc_runner::{AoC, Analyzer};
//!
//! #[derive(Analyzer)]
//! #[derive(AoC)]
//! pub(crate) struct Days(
//!   day01::Day01,
//!   day02::Day02,
//!   day03::Day03,
//!   day04::Day04,
//!   day05::Day05,
//!   day06::Day06,
//! );
//! ```
//!
//! ## Using a custom Analyzer
//! ```rust
//! use aoc_runner::{Day,Analyzer};
//! use derive_aoc_runner::{AoC, Analyzer};
//!
//! struct MyAnalyzer;
//!
//! impl Analyzer for MyAnalyzer {}
//!
//! #[derive(AoC)]
//! pub(crate) struct Days(
//!   day01::Day01,
//!   day02::Day02,
//!   day03::Day03,
//!   day04::Day04,
//!   day05::Day05,
//!   day06::Day06,
//! );
//!
//! impl Days {
//!     fn get_analyzer(&self) -> impl Analyzer {
//!         MyAnalyzer
//!     }
//! }
//! ```
//!
//! ## Run puzzles
//! ```rust
//! let runner = Days::new(); // creates new runner with Default::default() values for puzzles
//! runner.run_day( 1 /* day */,               "" /* input */, &mut runner.get_analyzer());
//! runner.run_part(1 /* day */, 2 /* part */, "" /* input */, &mut runner.get_analyzer());
//! runner.run_all(["", ""] /* inputs */,                      &mut runner.get_analyzer());
//! ```
//!

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsUnnamed};

/// Derives multiple utility methods for running Advent of Code puzzles on a struct
///
/// Derives
/// - `fn new() -> Self`
/// - `fn len(&self) -> usize`
/// - `fn is_empty(&self) -> bool`
/// - `fn run_day(&self, day: usize, input: &str, analyzer: &mut impl Analyzer)`
/// - `fn run_part(&self, day: usize, part: usize, input: &str, analyzer: &mut impl Analyzer)`
/// - `fn run_all<I: AsRef<str>>(&mut self, inputs: &[I])`
#[proc_macro_derive(AoC)]
pub fn derive_aoc(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    let usage = format!(
        "#[derive(AoC)] can currently only be applied to structs with unnamend fields.

Also, alls fields must implement aoc_runner::Day + Default.

Example:
#[derive(Aoc)]
struct Aoc2022(
    day01::Day01,
    day02::Day02,
);"
    );

    if let Data::Struct(s) = data {
        if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = s.fields {
            let fields_len = unnamed.len();
            let fields_ty = unnamed.iter().map(|f| &f.ty);
            let field_indices = (0..fields_len).map(syn::Index::from);

            let new_impl = {
                let doc = format!(
                    "Creates a new [{}], initialized with default values for all days",
                    ident
                );
                quote! {
                    #[doc = #doc]
                    pub fn new() -> Self {
                        Self(#(<#fields_ty>::default(),)*)
                    }
                }
            };

            let len_impl = {
                let doc = "Return the number of available days";
                quote! {
                    #[doc = #doc]
                    pub fn len(&self) -> usize {
                        #fields_len
                    }
                }
            };

            let is_empty_impl = {
                let doc = "Return true if the container hosts no days";
                quote! {
                    #[doc = #doc]
                    pub fn is_empty(&self) -> bool {
                        #fields_len == 0
                    }
                }
            };

            let run_day_impl = {
                let doc = "Runs both parts of a given day";
                quote! {
                    #[doc = #doc]
                    pub fn run_day(&mut self, day: usize, input: &str, analyzer: &mut impl Analyzer) {
                        self.run_part(day, None, input, analyzer);
                    }
                }
            };

            let run_part_impl = {
                let doc = "Runs one or both parts of a given day";
                quote! {
                    #[doc = #doc]
                    pub fn run_part(&mut self, day: usize, part: Option<usize>, input: &str, mut analyzer: &mut impl Analyzer) {
                        match day - 1 {
                            #( #field_indices => {
                                println!("Day {}", day);

                                analyzer.before_day(day);
                                analyzer.before_parse(day);
                                self.#field_indices.parse(input);
                                analyzer.after_parse(day);
                                match part {
                                    Some(1) => {
                                        analyzer.before_part(day, 1);
                                        let result = self.#field_indices.part1();
                                        analyzer.after_part(day, 1);
                                        self.#field_indices.print_part1(result);
                                    }
                                    Some(2) => {
                                        analyzer.before_part(day, 2);
                                        let result = self.#field_indices.part2();
                                        analyzer.after_part(day, 2);
                                        self.#field_indices.print_part2(result);
                                    }
                                    None => {
                                        analyzer.before_part(day, 1);
                                        let result = self.#field_indices.part1();
                                        analyzer.after_part(day, 1);
                                        self.#field_indices.print_part1(result);

                                        analyzer.before_part(day, 2);
                                        let result = self.#field_indices.part2();
                                        analyzer.after_part(day, 2);
                                        self.#field_indices.print_part2(result);
                                    }
                                    Some(part) => panic!("Invalid part: {}. Valid parts are: 1,2", part)
                                }
                                analyzer.after_day(day);
                            } )*
                            _ => panic!("Invalid day: {}. Valid days are 1..{}", day, #fields_len)
                        }
                    }
                }
            };

            let run_all_impl = {
                let doc = "Runs both parts of all available days";
                quote! {
                    #[doc = #doc]
                    pub fn run_all<I: AsRef<str>>(&mut self, inputs: &[I]) {
                        assert_eq!(inputs.len(), self.len());

                        let mut analyzer = self.get_analyzer();

                        analyzer.before_all();
                        for i in 1..=self.len() {
                            let input = inputs[i - 1].as_ref();
                            self.run_day(i, input, &mut analyzer);
                        }
                        analyzer.after_all();
                    }
                }
            };

            let run_some_impl = {
                let doc = "Runs both parts for all days where an input is given";
                quote! {
                    #[doc = #doc]
                    pub fn run_some<I: AsRef<str>>(&mut self, inputs: &[Option<I>]) {
                        assert_eq!(inputs.len(), self.len());

                        let mut analyzer = self.get_analyzer();

                        analyzer.before_all();
                        for i in 1..=self.len() {
                            if let Some(input) = &inputs[i - 1] {
                                self.run_day(i, input.as_ref(), &mut analyzer);
                            }
                        }
                        analyzer.after_all();
                    }
                }
            };

            let output = quote! {
                impl #ident {
                    #new_impl
                    #len_impl
                    #is_empty_impl
                    #run_day_impl
                    #run_part_impl
                    #run_all_impl
                    #run_some_impl
                }
            };

            // panic!("{}", output.to_string());
            output.into()
        } else {
            panic!("{}", usage)
        }
    } else {
        panic!("{}", usage)
    }
}

/// Derives a method for this struct that returns an [Analyzer][aoc_runner::Analyzer] instance
///
/// Derives
/// - `fn get_analyzer(&self) -> impl Analyzer`
#[proc_macro_derive(Analyzer)]
pub fn derive_get_analyzer(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let get_analyzer_impl = {
        let doc = "Creates a new analyzer to use during execution";
        quote! {
            #[doc = #doc]
            pub fn get_analyzer(&self) -> impl Analyzer {
                TimeAnalyzer::new()
            }
        }
    };

    let output = quote! {
        use aoc_runner::{TimeAnalyzer};
        impl #ident {
            #get_analyzer_impl
        }
    };

    output.into()
}
