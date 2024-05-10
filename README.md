# dist-sys

## Testing using Maelstrom

`./maelstrom/maelstrom test -w <test-name> --bin <path-to-bin> --nodes n1 --time-limit 10`

### Testing echo

`./maelstrom/maelstrom test -w echo --bin target/debug/echo --time-limit 10 --nodes 1`

### Testing uniqe-ids

`./maelstrom/maelstrom test -w unique-ids --bin target/debug/unique-ids --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition`
