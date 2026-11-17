# Drizzle User Guide

Lamda RED compiles Drizzle source into requirements engineering diagrams. Use
the PNG in documents and presentations. Keep the Drizzle file with the
requirements so reviewers can trace and revise the goals, risks, agents, and
relationships. Each build also produces an editable mxGraph XML file.

## Compile a diagram

Run these commands from the repository root with a recent stable Rust
toolchain. PNG rendering uses your system fonts. [Install Rust and Cargo](https://www.rust-lang.org/tools/install).

```sh
cargo run --locked
cargo run --locked -- sample/requirements.zz requirements
cargo run --locked -- --help
```

The first command compiles `sample.zz` to `output.png` and `output.xml`.
The second writes `requirements.png` and `requirements.xml`. The two
positional arguments are the input file and the output stem. The stem may
include a directory that already exists, such as `reports/requirements`.
Lamda RED appends `.png` and `.xml` to the stem. Omitting it selects `output`.

If your Windows Rust installation uses GNU, select that toolchain explicitly.
Windows MSVC builds require the Visual Studio C++ build tools. [Windows Rust setup](https://learn.microsoft.com/en-us/windows/dev-environment/rust/setup).

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal
cargo +stable-x86_64-pc-windows-gnu run --locked -- sample/requirements.zz requirements
```

### A first diagram

Save this as `first.zz` and run `cargo run --locked -- first.zz first`.

```drizzle
&& The tracking agent reports train speed and location.
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
```

The output has a hexagonal agent, a rectangular object, and a solid arrow
pointing to the train. The label above the arrow has two lines.

## Declare nodes

Write `ID = shape` so a connection can refer to the node. IDs are
case-sensitive, start with an ASCII letter, and accept letters, digits, and
single underscores between alphanumeric characters. `Train_1` is an ID.
The text inside the shape is its display label. Connections may appear before
or after their declarations. Use `&&` at the start of a line for a comment.
Blank lines and indentation are allowed.

You can also write shapes by themselves, such as `/Safe journey/`. Give a
shape an ID when you want to use it in a connection.

| Shape | Declaration | Result |
| --- | --- | --- |
| Goal | `G = /Safe journey/` | Parallelogram leaning right |
| Risk | `R = \Signal lost\` | Mirrored parallelogram |
| Agent | `A = <Train controller>` | Hexagon |
| Descriptive statement | `D = ~Doors are locked in motion~` | Trapezoid with a wider base |
| Object | `O = [Train]` | Rectangle |
| Action | `Send = (Send command)` | Ellipse |
| Note | `N = {Controller responsibility}` | Light green rectangle with a dashed outline |
| Cloud annotation | `C = {{Reason for the obstacle}}` | Cloud outline |
| Diamond | `Check = ^Speed confirmed?^` | Rhombus |

Prefix a shape with `!` to thicken its outline. The reference notation uses
this for leaves such as requirements and assumptions. The label keeps its
normal weight.

```drizzle
/Goal/
!/Leaf requirement/
~Descriptive~
<Agent 1>
```

### Labels and line breaks

Labels accept UTF-8, including right-to-left text. The renderer measures the
text, centers it inside the shape, and sizes the shape to fit. Long labels
wrap at words when possible. A long word can wrap within the word. Shape
labels show up to four lines, and connection labels show up to three. Text
that exceeds the limit ends with `...`. The full label remains in the source.

| Syntax | Result |
| --- | --- |
| `/Accurate estimate\nof speed and position/` | A goal label with a line break |
| `[C:\\Logs]` | A rectangle labeled `C:\Logs` |
| `<Speed \> zero>` | An agent labeled `Speed > zero` |
| `/Input \/ output/` | A goal labeled `Input / output` |
| `~Value \~ threshold~` | A trapezoid labeled `Value ~ threshold` |

Write `\n` for a forced line break, `\\` for a literal backslash, and put a
backslash before a delimiter you want to display inside a label.

## Connect nodes

Use named nodes on either side of the operator. The two middle characters
select the line style. The outer `<` and `>` add arrowheads at the left and
right ends respectively. Use spaces between IDs and operators for
readability when needed.

| Direction | Solid | Dashed |
| --- | --- | --- |
| No arrowheads | `A __ B` | `A -- B` |
| Points to B | `A__>B` | `A-->B` |
| Points to A | `A<__B` | `A<--B` |
| Points to both ends | `A<__>B` | `A<-->B` |

Add a colon after the connection and its options to write a label. Two
connections between the same nodes are drawn on separate tracks so each can
carry a different label. A connection from a node to itself forms a loop.

```drizzle
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train --> Train : Refresh state
```

### Add notation to a connection

| Option | Example | Result |
| --- | --- | --- |
| `[o]` | `A __ B [o]` | An open circle on the line |
| `[*]` | `A __> B [*]` | A filled circle on the line |
| `[zigzag]` | `A __ B [zigzag]` | A zigzag for a conflict |
| `[bar]` | `Risk --> Goal [bar]` | A crossbar for an obstacle |
| `[ortho]` | `A __> B [ortho]` | A route built from horizontal and vertical segments |

Use one circle, zigzag, or crossbar per connection. You can combine that
marker with `[ortho]` and with either line style. A crossbar on a
single-arrow connection appears near the arrowhead. A crossbar on a plain
or double-arrow connection appears near the middle of the route.

### Join several sources

List the source IDs with commas to draw a shared connection. An open or
filled circle on a branch sits at the junction. A right arrowhead sits on
the shared segment into the target. A left arrowhead points into each
source. Write separate connections when each refinement needs its own
arrowhead or marker.

```drizzle
layout up
Goal @1,0 = /Safe departure/
Software @0,1 = !/Verify the lock signal/
Assumption @2,1 = !/The door sensor works/

Software, Assumption __> Goal [o]
```

The two leaf nodes join a common line with an open circle and a single
arrow into the goal above them.

## Set the layout and routing

| Control | Example | Result |
| --- | --- | --- |
| Direction | `layout right` | Automatic flow from left to right. This is the default |
| Direction | `layout left` | Automatic flow from right to left |
| Direction | `layout up` | Automatic flow from bottom to top with right-angled routes |
| Direction | `layout down` | Automatic flow from top to bottom with right-angled routes |
| Grid placement | `Goal @2,0 = /Goal/` | Places the shape in column 2, row 0 |
| Outer route | `A --> B [bar] via bottom` | Sends the connection around the bottom of the diagram |

Write at most one layout directive per diagram. Grid coordinates start at
zero, with columns increasing to the right and rows increasing downward.
Each explicit cell holds one node. Lamda RED sizes each row and column to
fit its shapes and sizes the PNG canvas to include the diagram. You can mix
automatic and explicit placement.

Write `via right`, `via left`, `via top`, or `via bottom` after the connection
options and before its label. This is useful when a conflict or an
obstruction needs to bypass other relationships.

```drizzle
layout up
Login @0,0 = /User can sign in/
Blocked @0,1 = \User cannot sign in\
Recover @0,2 = /Recover using a phone number/

Recover __> Blocked
Blocked __> Login
Recover __> Login [zigzag] via right : A non-user might gain access
```

The first two connections point up through the tree. The conflict follows
the right side and points back to the goal.

## Reference diagrams

Each example shows the result of compiling the linked Drizzle file. Run
`cargo run --locked -- sample/de-morgan.zz de-morgan` for the De Morgan
diagram, or substitute another example path and output stem.

### Goal notation and obstacles

The heavy outline distinguishes a leaf, the trapezoid records a descriptive
statement, and the mirrored risk shape points to a goal through an obstacle
bar. A cloud holds the explanatory note. [Drizzle source](../sample/notation.zz).

![Goal and risk notation with a cloud, trapezoid, hexagon, and crossbar](images/notation.png)

### Requirement refinement

Independent connections let the refined requirement and leaf requirements
carry their own markers and arrowheads. [Drizzle source](../sample/requirements.zz).

![Bottom-up refinement tree with three thick-outlined leaves](images/requirements.png)

### Rail goal conflicts

Branches join several refinements at one junction. The zigzags connect
conflicting goals across the tree. [Drizzle source](../sample/rail-conflicts.zz).

![Rail goal diagram with shared circle junctions and two zigzag conflicts](images/rail-conflicts.png)

### Authentication risk

Mirrored shapes show the problems that prevent sign-in. The outer zigzag
shows a possible conflict introduced by a recovery option. [Drizzle source](../sample/auth-risk.zz).

![Authentication risk tree with a zigzag route around its right side](images/auth-risk.png)

### De Morgan goal and risk diagram

The source explicitly declares the goal and its negation, the shared
refinement branch, and the three dashed obstructions. [Drizzle source](../sample/de-morgan.zz).

![Goal and negated goal with a shared branch and dashed crossbar relationships](images/de-morgan.png)

### Agent monitoring and control

The hexagonal agent receives an attribute from the first object and
controls an attribute of the second. [Drizzle source](../sample/object-agent.zz).

![An input object connects to an agent, which then connects to an output object](images/object-agent.png)

### Train control

Solid and dashed lines, separate arrows for measurements and commands,
an action ellipse, and a responsibility note appear in one diagram.
[Drizzle source](../sample.zz).

![Train control requirements with agents, objects, an action, and a responsibility note](../output.png)

## Editor and diagnostics

The VS Code extension source in `drizzle/` highlights `.zz`, `.drizzle`,
and `.rd` files. To try the current grammar, open that folder in VS Code and
press **F5** to start the Extension Development Host. Open your diagram
source in the host window. [VS Code extension documentation](https://code.visualstudio.com/api/get-started/your-first-extension).

| Message or symptom | How to resolve it |
| --- | --- |
| `unknown node 'X'` | Declare `X` with the same capitalization or correct the reference. |
| `duplicate ID` | Give each declared node a unique ID. |
| `cell @… is already occupied` | Assign one of the nodes to another grid cell. |
| `invalid Drizzle syntax` | Use the file, line, and column in the message to find the delimiter, operator, or misplaced option. |
| `diagram is too large for PNG output` | Bring sparse grid positions closer or split the model into diagrams. |
| `no system fonts found` | Install a sans-serif font and rerun the compiler. |
| `link.exe not found` | Install the MSVC build tools or run the GNU command above. |
