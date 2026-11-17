<div align="center">
    <img src="../images/logo_drizzle_language.png" width="300" height="300" />
    <h1>A Simple RED Language Support</h1>
</div>

Drizzle adds syntax highlighting for Lamda RED requirements diagrams.

## Demo

<div align="center">
    <img src="../images/drizzle_demo.png" />
</div>

## Sample

```drizzle
&& Train control requirements
Estimate @0,0 = /Accurate estimate\nof speed & position/
Tracking @1,0 = <Tracking system>
Train @2,0 = [Train]
Duty @0,1 = {Train controller responsibility\nCompute safe acceleration for all trains between this station and the next.}
Safe @0,2 = /Safe command\nmessage/
Controller @1,2 = <Train controller>
Timing @0,3 = /Command in time/
Send @1,3 = (Send\ncommand)
Command @2,3 = [Command]

Estimate __ Tracking [o]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train __> Controller : As monitor\nmeasuredSpeed\nmeasuredLocation
Controller __> Command : As control\ncommandedSpeed\ncommandedAccel
Safe __ Controller [o]
Timing __ Controller [o] : Responsibility
Controller __ Send [o] : Performance
Duty --> Controller
```

## Output

<div align="center">
    <img src="../output.png" />
</div>

See the [Lamda RED syntax guide](../README.md#drizzle-syntax) for goal trees,
shape types, connection markers, and layout controls.

## Release Notes

### 1.0.0

Initial release of Drizzle
