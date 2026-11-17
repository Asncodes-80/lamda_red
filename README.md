<div align="center">
    <img src="./images/logo_lamda_red.png" width="300" height="300" />
    <p>Draw requirements engineering diagrams from Drizzle source files.</p>
</div>

[Read the Drizzle User Guide](./docs/drizzle-guide.md) for the full syntax
reference, runnable examples, and rendered diagrams.

## Running

Build with Rust and Cargo. PNG rendering uses the system fonts.

```sh
cargo run --locked
cargo run --locked -- sample/requirements.zz requirements
```

The first command compiles [the train control example](./sample.zz) into
[output.png](./output.png) and [output.xml](./output.xml). The second compiles
the requirement refinement example into `requirements.png` and
`requirements.xml`. For Windows build commands, see the [setup section in the user guide](./docs/drizzle-guide.md#compile-a-diagram).

## Drizzle syntax

```drizzle
layout up
Goal @1,0 = /Safe departure/
Software @0,1 = !/Verify the lock signal/
Assumption @2,1 = !/The door sensor works/

Software, Assumption __> Goal [o]
```

This draws two thick-outlined leaf nodes that join at an open circle and
point up to the goal. `__` draws a solid line, `--` draws a dashed line, and
`<` or `>` adds an arrowhead to the corresponding end.

## Roadmap

- Create a state machine diagram for the Lamda RED Pest grammar.
- Build a web service with a React app that requests rendered PNGs from a Rust server.

## License

This project is licensed under the [GNU GPLV3](./LICENSE).
