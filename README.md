# Text Searcher
This is CLI tool to search for a specific word or specific sentence in specified text file. The tool is programmed in Rust so it has better execution speed than other tools.

The command will be of form
`minigrep <search_term> <file_path>`

To collect the results in a file you can write the command has
`minigrep <search_term> <file_path> > <output_file_path>`

There are 3 arguments:
- `<search_term>`: The word or sentence to search for.
- `<file_path>`: The path to the text file to search in.
- `IGNORE_CASE`: This flag will ignore the case of the search term.

To use IGNORE_CASE argument you can write the command as
`IGNORE_CASE=1 minigrep <search_term> <file_path>`
