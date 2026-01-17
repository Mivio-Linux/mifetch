# MiFetch
## Mini fetch written in ~27 lines of code. Made just for fun
if you want to make custom ASCII, then you need open/create `/etc/mifetch/ascii.txt` and make ascii.
To colorize in start every line add:
`$1` - red
`$2` - green
`$3` - blue
`$4` - yellow
`$5` - purple
to add custom colors, you can add to `colors` (in 3 line) your colors.
# build
```bash
git clone https://gitea.com/miviodev/mifetch.git
cd mifetch
cargo build -r
```
binary in `target/release/
