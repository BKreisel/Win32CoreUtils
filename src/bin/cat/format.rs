pub fn number_nonblank_lines(contents: String) -> String {String::from("UNIMPLEMENTED")}
pub fn show_ends(contents: String) -> String {String::from("UNIMPLEMENTED")}
pub fn number_lines(contents: String) -> String {String::from("UNIMPLEMENTED")}
pub fn squeeze_blank(contents: String) -> String {String::from("UNIMPLEMENTED")}
pub fn show_tabs(contents: String) -> String {String::from("UNIMPLEMENTED")}
pub fn show_nonprinting(contents: String) -> String {String::from("UNIMPLEMENTED")}

#[cfg(test)]
mod tests {
    use crate::format;
    use coreutils::test_utils;

    #[test]
    fn number_nonblank() {
        assert_eq!(
            format::number_nonblank_lines(test_utils::MULTI_BLANK.to_string()),
            "\t 1 this is a file\r\n\
            \r\n\
            \r\n\
            \t 2 that has\r\n\
            \r\n\
            \r\n\
            \t 3 multiple blank lines.\r\n"
        )
    }

    #[test]
    fn show_ends() {
        assert_eq!(
            format::show_ends(test_utils::MULTI.to_string()),
            "this is a text file$\r\n\
            with multiple lines$\r\n\
            of text to show.$\r\n"
        )
    }

    #[test]
    fn number_lines() {
        assert_eq!(
            format::number_lines(test_utils::BLANK.to_string()),
            "\t 1 this is a file\r\n\
            \t 2 \r\n\
            \t 3 that has\r\n\
            \t 4 \r\n\
            \t 5 blank lines.\r\n"
        )
    }

    #[test]
    fn squeeze_blank() {
        assert_eq!(
            format::squeeze_blank(test_utils::MULTI_BLANK.to_string()),
            "this is a file\r\n\
            \r\n\
            that has\r\n\
            \r\n\
            multiple blank lines.\r\n"
        )
    }

    #[test]
    fn show_tabs() {
        assert_eq!(
            format::show_tabs(test_utils::MULTI.to_string()),
            "This^Itext^Ihas^Isome^Itabs^Iin^Iit.\r\n"
        )
    }

    #[test]
    fn show_nonprinting() {
        assert_eq!(
            format::show_nonprinting(test_utils::NON_PRINTABLE.to_string()),
            "Lots \t of ^@ control ^@^@ characters.^M\n"
        )
    }
}