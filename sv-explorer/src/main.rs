use std::path::Path;
use sv_explorer_args::parse;

fn main() {
    let filelist = parse(Path::new("filelist.f"));
    let (defines, includes, files) = (filelist.defines, filelist.includes, filelist.files);
}
