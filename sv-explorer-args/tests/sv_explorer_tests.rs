use std::path::Path;

use sv_explorer_args::ast::parse;

#[test]
fn parse_filelist() {
    let filelist = Path::new("tests/inputs/test_filelist.f");
    let result = parse(filelist);
    // TODO: Add the error checking for the filelist.f
}
