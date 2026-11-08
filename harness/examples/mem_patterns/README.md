# Mem pattern run

This folder is a cleaned copy of one run against `Qwen3.6-35B-A3B-NVFP4`.

- `request-1.json` is the request body.

- `reply-1.txt` is the raw reply, with the code fence.

- `main.rs` is the generated program.

- `run.log` is the console output from the tool.

The check command was:

```sh
rustc main.rs && ./main
```

The run passed on attempt 1 with `finish_reason: stop` and 1426 completion tokens.

`main.rs` compiles with an `unused_mut` warning in `demo_mem_drop`.
