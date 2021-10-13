# Modality probe RTIC tracing integration

A proof of concept Modality probe integration with [RTIC](https://rtic.rs/).

Currently the integration is realized as as a set of in-tree patches
for [rtic-syntax](https://github.com/rtic-rs/rtic-syntax) and [cortex-m-rtic](https://github.com/rtic-rs/cortex-m-rtic).
These will eventually be replaced by an out-of-tree integration solution.

A new custom attribute is provided (`#[modality_probe(...)]`), which when applied to a task
will synthesize a Modality probe instance in the the task's local resources.

## Getting Started

1. Install Modality debian
2. Update your `Cargo.toml` dependencies to point to our `cortex-m-rtic` fork
  ```toml
  cortex-m-rtic = { git = "https://github.com/auxoncorp/modality-rtic.git" }
  ```
3. Add `modality-probe-sys` crate dependency to your `Cargo.toml`
  ```toml
  [dependencies.modality-probe-sys]
  # If using the Modality debian package
  path = "/usr/share/modality/modality-probe-sys"
  # If using the Modality tarball package
  # For example, if Modality is extracted to /usr/local/modality:
  # export PATH=$PATH:/usr/local/modality/bin
  # export MODALITY_PROBE_VERSION=$(cat /usr/local/modality/VERSION)
  # export MODALITY_PROBE_LIBRARY_DIR=/usr/local/modality/lib/<target-triple>
  #path = "/usr/local/modality/rust/modality-probe-sys"
  features = ["static"]
  default-features = false
  ```
4. Decorate tasks with `modality_probe` attribute
  ```rust
  // name: Probe name used when generating a manifest entry for the probe
  // size: Probe storage buffer size
  // local_name: Optional local variable name, defaults to 'probe'
  #[task]
  #[modality_probe(name = PROBE_FOO, size = 1024)]
  fn foo(ctx: foo::Context) {
      record!(ctx.local.probe, EVENT_FOO, "Event FOO happened", tags!("foo", "RTIC"));
  }

  #[task]
  #[modality_probe(name = PROBE_BAR, size = 1024, local_name = p)]
  fn bar(ctx: bar::Context) {
      record_w_u32!(ctx.local.p, EVENT_BAR, 1_u32, "Event BAR happened", tags!("bar", "RTIC"));
  }
  ```

## Example

1. Install QEMU
  ```bash
  sudo apt install qemu-system-arm
  ```
2. Install `thumbv7m-none-eabi` target
  ```bash
  rustup target add thumbv7m-none-eabi
  ```
3. Build the example
  ```bash
  cd example/
  cargo build
  ```
4. Run it
  ```bash
  cargo run
  ```
  ```text
  Initializing
  foo
  foo
  foo
  foo
  Comms task should send a ModalityProbe report size=69
  foo
  foo
  foo
  foo
  Comms task should send a ModalityProbe report size=61
  foo
  foo
  foo
  foo
  Comms task should send a ModalityProbe report size=61
  foo
  foo
  foo
  foo
  Comms task should send a ModalityProbe report size=61
  foo
  foo
  foo
  foo
  All done
  ```
