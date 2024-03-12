# PDF Filename Parser 
Parser that listens on new files in a specific directory, looks up the correct title in the pdf metadata and changes the file name.  

Source and target directories are configurable.  
Default location:  
`$HOME/.config/pdfTitleParser/config.toml`

## How to Run
Either in dev mode:
`cargo run`

or production mode:
`cargo build -r`

