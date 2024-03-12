# PDF Filename Parser 
Parser that listens on new files in a specific directory, looks up the correct title in the pdf metadata and changes the file name.  
**Use Case:** When downloading from science databases like IEEE the pdf is named with the DOI or ISBN instead of the actual name.  

Source and target directories are configurable.  
Default location:  
`$HOME/.config/pdfTitleParser/config.toml`  
with the default paths:  
`$HOME/pdfs/`

## How to Run
The application must be run manually, but then it listenes on the directory specified in the configuration.  
Either in dev mode:  
`cargo run -- [OPTIONAL] <src_dir> <target_dir>`  

or production mode:  
`cargo build -r`  
and then run the executable in the target directory with:
```bash
./pdfparser [OPTIONAL] <src_dir> <target_dir>
```

**IMPORTANT:**  
When providing the src and target directory argument the binary configuration is updated and when running the second time the optional arguments must not be set.  

## TODO:
* generall: lots of improvements
* better cli description
* improve config path parsing
* modulate the config logic
* functionality to change the config location
* better error handling: a lot of work arounds due to the buffering of the downloading/ copy/ move operations
  * therefore a not quite nice sleep on certain event conditions