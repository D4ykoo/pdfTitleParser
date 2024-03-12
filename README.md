# PDF Filename Parser 
Parser that listens on new files in a specific directory, looks up the correct title in the pdf metadata and changes the file name.  

Source and target directories are configurable.  
Default location:  
`$HOME/.config/pdfTitleParser/config.toml`

## How to Run
Either in dev mode:  
`cargo run -- [OPTIONAL] <src_dir> <target_dir>`  

or production mode:  
`cargo build -r`  
and then run the executable in the target directory with:
```bash
./pdfparser [OPTIONAL] <src_dir> <target_dir>
```

## TODO:
* generall: lots of improvements
* better cli description
* improve config path parsing
* modulate the config logic
* functionality to change the config location