# lat-lon-to-county

A small command line tool build for the <a href="https://github.com/DToL-Plant-Working-Group/collections">Darwin Tree of Life Plant Working Group</a>, as I was getting into pains parsing the data from COPO.

May have wider use. Shout out to the geo-dojo API for not blocking my IP address, and making this data pretty much freely available at their expense. We shall see how long this lasts.

The build requires Rust.

```bash
git clone https://github.com/Euphrasiologist/lat-lon-to-county
cd lat-lon-to-county
cargo build --release
```

Executable in `./target/release/geodojo_county`.

## Usage

```
geodojo_county 0.1.0
Max Brown <mb39@sanger.ac.uk>
Get UK counties from lat-long data.

USAGE:
    geodojo_county --lat <lat> --lon <lon>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --lat <lat>    The input latitude.
        --lon <lon>    The input longitude.
```

Or simply:

`geodojo_county --lat 55.9459201493332 --lon -2.72697188954985`

