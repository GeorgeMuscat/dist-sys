# dist-sys

## Testing using Maelstrom

`./maelstrom/maelstrom test -w <test-name> --bin <path-to-bin> --nodes n1 --time-limit 10`

### Testing echo

`./maelstrom/maelstrom test -w echo --bin target/debug/echo --time-limit 10 --nodes 1`

### Testing unique-ids

`./maelstrom/maelstrom test -w unique-ids --bin target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition`

### Testing single node broadcast

`./maelstrom/maelstrom test -w broadcast --bin target/debug/broadcast --node-count 1 --time-limit 20 --rate 10`
