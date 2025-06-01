/*!
Lexer and Parser to capture SystemVerilog includes, defines, and file inputs for parsing.
All of the arguments are captured in a single filelist, such as `filelist.f`. This file can contain
common dash flags that various EDA tools support.

Relative paths are evaluated from the location of the input file.

The following file arguments are allowed:
- [+incdir+](#example-include-directory)
- [+define+](#example-defines)
- [File Paths](#example-file-paths)
- [-v](#example-verilog-library)
- [-y](#example-verilog-module-search-directory)

# Example: Include Directory
Use the `+incdir+` flag to pass in include directories to be used for parsing
```text
+incdir+../../include_directory/
```

# Example: Defines
Use the `+define+` flag to pass in defines needed for parsing
```text
+define+GATE_SIM
+define+TEST_NAME=check_performance
```

# Example: File Paths
Pass in SystemVerilog files directory to be captured for parsing.
```text
../../../alu.sv
/home/users/mynameisjeff/sv/arbiter.sv
```

# Example: Verilog Library
Use the `-v` flag to pass in verilog library files.
```text
-v ../sv_lib/
```

# Example: Verilog Module Search Directory
Use the `-y` flag to pass in verilog library directories to search
```text
-y ../module_directory/
```
*/

pub mod ast;
pub mod lexer;
