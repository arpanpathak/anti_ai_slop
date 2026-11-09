# Experiments

Each experiment is a program in `src/bin/`. It reuses `src/sandbox.rs` unchanged and saves
its result as a new file, `experiments/<name>/<UTC time>.txt`. The file starts with the time,
the commit and the kernel. A run never overwrites an earlier result, so every file here is
the record of one run.

| Program | Question |
| --- | --- |
| `cgroup-limits` | Does a cgroup stop a flood of processes and a full `/work`, which the sandbox alone allows? |
| `escape-mutations` | Do 104 variations of the escape attempts leave anything on the host? |
| `host-matrix` | On this host, which kernel settings and bubblewrap version apply, and does the sandbox start? |
| `typed-check` | How often does the model's own check pass a program that prints the wrong answer? |
| `output-injection` | Does an instruction printed by a failed check change the model's next reply? |

Run one from the `harness` directory:

```sh
cargo run --release --bin cgroup-limits
API_KEY=... cargo run --release --bin typed-check
```

`typed-check` and `output-injection` call the model server and need `API_KEY`. In
`escape-mutations`, every attempt also runs once without the sandbox. Those runs show that the
judge notices a real escape.
