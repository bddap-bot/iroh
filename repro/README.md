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
program's only argument.

`Cargo.lock` pins iroh 1.0.3. To run against another iroh source, give the
same patch flags to an update and to the build; a patch whose version differs
from the locked one is not used until the lock takes it:

```sh
patch=(--config 'patch.crates-io.iroh.git="..."' --config 'patch.crates-io.iroh.rev="..."')
cargo "${patch[@]}" update -p iroh
cargo "${patch[@]}" build --release
```

The build log proves which source was compiled: `Compiling iroh v… (https://…)`
names the patched one, and `warning: patch … was not used in the crate graph`
means the lock still pins 1.0.3.
