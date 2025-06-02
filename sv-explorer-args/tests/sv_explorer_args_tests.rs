use std::path::Path;
use sv_explorer_args::ast::*;

#[test]
fn parse_filelist() {
    let filelist = Path::new("tests/inputs/test_filelist.f");
    let result = parse(filelist);
    let (defines, includes, files) = (result.defines, result.includes, result.files);
    assert_eq!(
        defines,
        vec![
            DefineArgs {
                define: "DEFINE1".to_string(),
                arg: Some("true".to_string())
            },
            DefineArgs {
                define: "DEFINE3".to_string(),
                arg: None
            },
            DefineArgs {
                define: "DEFINE2".to_string(),
                arg: Some("true".to_string())
            }
        ]
    );
    assert_eq!(
        includes,
        vec![IncludeArgs {
            include_dir: "../sv/".to_string()
        },]
    );
    assert_eq!(
        files,
        vec![
            FileArgs {
                file: "../sv/adder.sv".to_string(),
            },
            FileArgs {
                file: "../sv/mult.sv".to_string(),
            },
            FileArgs {file : "/generated-src/chipyard.to.SomeConfig/gen-collateral/FixedClockBroadcast_4.sv".to_string()}
        ],
    );
}
