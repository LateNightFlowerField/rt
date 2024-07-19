## Build

Requires [mocp](https://moc.daper.net) be installed.

```
cargo build --release
```
## Usage

`rt` takes 5 arguments `limit padding move_speed hold_time reverse`


Shows 30 characters at a time, adds 10 spaces between start and end, moves 1 character every 300 milliseconds, pauses for 1 second at beginning of title, moves from right to left.  

```
./rt 30 10 300 1000 false
```


## Example Polybar config:

```
[module/song]
type=custom/script
label = %output%
interval = 1
exec = PATH/TO/rt/target/release/rt
tail = true
```


