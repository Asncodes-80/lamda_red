<div align="center">
    <img src="./images/logo_lamda_red.png" width="300" height="300" />
    <p>LamdaRED is a simple way to create your Requirement Engineering Diagrams
    without using a mouse or the complex interface of draw.io just by using
    straightforward syntax!</p>
</div>

## Syntax

```txt
/A Requirement Goal/
\A Requirement Risk Shape\
<Agent processes over a specific Goal>
```

## TODO Level 1

+ [ ] Create State Machine diagram for `lamda_red` pest grammar,
+ [x] Make it standalone instead of import exported `.xml` file to draw.io app
    + [x] Create a png exporter
    + [x] Create three shapes
        + [x] Hexagon
        + [x] Parallelogram
        + [x] Flipped Parallelogram
    + [x] Label issue, position of start and end
    + [x] Multiline Label issue
    + [x] Centerline diagram
    + [x] Create compatible (surface) paper to generate `png` with proper
          `width` and `height` size
    + [x] Dynamic width and height object size (Parsing and export section)
    + [ ] Dynamic x object proximity
    + [x] Dynamic y object proximity
    + [ ] Dynamic page surface doesn't work with drizzle file lines.
      + [ ] Implement it with size of (x, y, width, height) every shape
+ [ ] Create line between two and a few objects
    + [ ] Connection and communication between levels
+ [ ] Make diagram stateful with any position situation (with other shapes and
      their connectivity)
+ [x] Define comment rule in `lamda_red` pest grammar.
+ [x] VSCode Drizzle Extension Library to Syntax-Highlighting
    + [x] Create syntax highlighter for `.zz` extension files
+ [ ] Syntax error in single quotation

## TODO Level 2

+ [ ] Add CLI feature
  + [ ] Read file from another path like a standalone software
+ [ ] Add theme color support
+ [ ] Make it web-based service
+ [ ] React App, it can request to a Rust server
+ [ ] Serve `png` as production of diagrams into server-client


[Official](https://jgraph.github.io/mxgraph/docs/js-api/files/model/mxGraphModel-js.html)
MXGraphModel documentation.

## Running

```sh
cargo update # Fetch dependencies 
cargo run # Compiling
```

## License

This project is licensed under the [GNU GPLV3](./LICENSE).