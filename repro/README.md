# Link-local IPv6 scope loss

Two endpoints in one network namespace, joined by a veth pair carrying
`fe80::1` and `fe80::2`. The client dials `[fe80::1%<ifindex>]` through the
public API; the program prints the address the kernel delivers, the address
iroh records for the incoming connection, and whether the handshake completes.

Build with network access, then run inside a fresh user+network namespace:

```sh
cd repro
cargo build --release
unshare -rn ./run.sh "$CARGO_TARGET_DIR/release/repro"
```

`run.sh` creates the veth pair and passes the dialing interface's index as the
program's only argument. To run against another iroh source, add cargo
`--config 'patch.crates-io.iroh.git="..."' --config 'patch.crates-io.iroh.rev="..."'`
to the build.
