# rs-spodgi

A(n incomplete) counterpart of [spodgi](https://github.com/pangenome/spodgi) implemented in Rust.

The functionality implemented is a line-by-line converter from [GFA](https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md) to [RDF (turtle)](https://www.w3.org/TR/turtle/), which avoids loading the entire input graph in memory.

### How to build
```
git clone --recursive https://github.com/AndreaGuarracino/rs-spodgi
cd rs-spodgi
cargo build --release
./target/release/rs-spodgi -h
```
```
gfa2rdf 0.1.0
Andrea Guarracino
GFA to RDF converter

USAGE:
    rs-spodgi --gfa <FILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --gfa <FILE>    GFA input file to convert
```

### How to run

```
./target/release/rs-spodgi -g test/t_small.gfa > t_small.ttl
```

### docker

You can build a docker image locally using the `Dockerfile`:

```
git clone --recursive https://github.com/AndreaGuarracino/rs-spodgi
cd rs-spodgi
docker build . --file Dockerfile --tag rs-spodgi
```

and run it with

```
docker run --rm -it -v ${PWD}/test/:/test rs-spodgi --gfa test/t.gfa
```
