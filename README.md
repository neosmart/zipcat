## `zipcat`

`zipcat` is a command-line utility that lets you pipe the contents of one or more compressed files to `stdout`, letting you easily search the contents of zip archives and more. `zipcat` has full support for actual zip archives (not just gz/deflate-compressed content), and provides options to optionally filter the files included/excluded from the enumeration.

### Installation

`zipcat` binaries are available via `cargo`, the rust package manager, as follows:

`cargo install zipcat`

Downloads for Windows are also available pre-compiled from NeoSmart Technologies at the `zipcat` homepage at https://neosmart.net/zipcat/

### Usage

    Usage: zipcat ZIPFILE [options]
    Pipes content of compressed file(s) within a zip archive to stdout
    
    Options:
        -h, --help          print this help menu
        -s, --silent        suppress file names from being sent to stderr
        -x, --exclude PATTERN
                            exclude file(s) matching pattern (can use more than
                            once)
        -i, --include PATTERN
                            include only file(s) matching pattern (can use more
                            than once)


By default, `zipcat` will pipe the contents of all files it encounters in the zip archive. Contents are written to `stdout`, prefixed with a single line containing the filename that is output to `stderr` (so it is always safe to perform operations on `stdout`). This behavior can be disabled via the `--silent` flag.

For pattern matching, `zipcat` supports filtering which files will be printed to `stdout` via the `--include` and `--exclude` command line arguments (which may be used repeatedly to add additional include/exclude patterns). Not providing any `--include` filters is synonymous with a default `*` include filter to include all files from the archive in the output. `zipcat` uses [rust's `glob` library](https://doc.rust-lang.org/glob/glob/index.html) for pattern matching include/exclude filters. Filters should match the entire path within the archive (no leading `.` or `/` is required), rather than being a partial match. `*` will match any pattern in a filename, and `**` can be used to glob across sudirectories.