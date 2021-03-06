use std::str::FromStr;
use std::{fmt, process, io::{self, Write}};

use tokei::{Language, LanguageType};
use input::Format;

pub const FALLBACK_ROW_LEN: usize = 79;
const NO_LANG_HEADER_ROW_LEN: usize = 67;
const NO_LANG_ROW_LEN: usize = 61;
const NO_LANG_ROW_LEN_NO_SPACES: usize = 54;

pub fn crate_version() -> String {
    if Format::supported().is_empty() {
        format!("{} compiled without serialization formats.", crate_version!())
    } else {
        format!(
            "{} compiled with serialization support: {}",
            crate_version!(),
            Format::supported().join(", ")
        )
    }
}

pub fn setup_logger(verbose_option: u64) {
    use log::LevelFilter;

    let mut builder = ::env_logger::Builder::new();

    let filter_level = match verbose_option {
        1 => LevelFilter::Warn,
        2 => LevelFilter::Debug,
        3 => LevelFilter::Trace,
        _ => LevelFilter::Error,
    };

    builder.filter(None, filter_level);
    builder.init();
}

pub fn print_input_parse_failure(input_filename: &str) {
    eprintln!("Error:\n Failed to parse input file: {}", input_filename);

    let not_supported = Format::not_supported();
    if !not_supported.is_empty() {
        eprintln!("
This version of tokei was compiled without serialization support for the following formats:

    {not_supported}

You may want to install any comma separated combination of {all:?}:

    cargo install tokei --features {all:?}

Or use the 'all' feature:

    cargo install tokei --features all
    \n",
            not_supported = not_supported.join(", "),
            // no space after comma to ease copypaste
            all = self::Format::all_feature_names().join(",")
        );
    }
}

pub fn print_supported_languages() {
    for key in LanguageType::list() {
        println!("{:<25}", key);
    }
}

pub fn parse_or_exit<T>(s: &str) -> T
    where T: FromStr,
          T::Err: fmt::Display
{
    T::from_str(s).unwrap_or_else(|e| {
        eprintln!("Error:\n{}", e);
        process::exit(1);
    })
}

pub fn print_header<W: Write>(sink: &mut W, row: &str, columns: usize)
    -> io::Result<()>
{
    writeln!(sink, "{}", row)?;
    writeln!(sink, " {:<6$} {:>12} {:>12} {:>12} {:>12} {:>12}",
                "Language",
                "Files",
                "Lines",
                "Code",
                "Comments",
                "Blanks",
                columns - NO_LANG_HEADER_ROW_LEN)?;

    writeln!(sink, "{}", row)
}

pub fn print_results<'a, I, W>(sink: &mut W, row: &str, languages: I, list_files: bool)
    -> io::Result<()>
    where I: Iterator<Item = (&'a LanguageType, &'a Language)>,
          W: Write,
{
    let path_len = row.len() - NO_LANG_ROW_LEN_NO_SPACES;
    let lang_section_len = row.len() ;
    for (name, language) in languages.filter(isnt_empty) {
        print_language(sink, lang_section_len, language, name.name())?;

        if list_files {
            writeln!(sink, "{}", row)?;
            for stat in &language.stats {
                writeln!(sink, "{:1$}", stat, path_len)?;
            }
            writeln!(sink, "{}", row)?;
        }
    }

    Ok(())
}

pub fn isnt_empty(&(_, language): &(&LanguageType, &Language)) -> bool {
    !language.is_empty()
}

pub fn print_language<W>(sink: &mut W,
                     lang_section_len: usize,
                     language: &Language,
                     name: &str)
    -> io::Result<()>
    where W: Write,
{
    writeln!(sink,
             " {:<len$} {:>6} {:>12} {:>12} {:>12} {:>12}",
             name,
             language.stats.len(),
             language.lines,
             language.code,
             language.comments,
             language.blanks,
             len = lang_section_len - NO_LANG_ROW_LEN)
}

