## extension-eyre

Re-export of color-eyre that introduces Extensions (type-map) to eyre::Report.

This is a simple wrapper around `color_eyre` with everything equal,
and just introduces traits `ExtensionExt`, `Extension` for adding and accessing custom data.

To learn more about `color-eyre`, see their [documentation](https://crates.io/crates/color-eyre).

## TLDR

`extension_eyre` helps you add extensions of any type to your error like this:

![custom section example](https://raw.githubusercontent.com/yaahc/extension-eyre/master/pictures/custom_section.png)

## Setup

Add the following to your toml file:

```toml
[dependencies]
extension-eyre = "0.1"
```

And install the panic and error report handlers:

```rust
use extension_eyre::eyre::Result;

fn main() -> Result<()> {
    extension_eyre::install()?;

    // ...
    # Ok(())
}
```

## Features

For all the features introduced by `color-eyre`, please see their [documentation](https://crates.io/crates/color-eyre).

### Adding extensions of any type to error reports via [`ExtensionExt`] trait

The crate exposes `ExtensionExt` trait for adding and
`Extension` trait for accessing extra data to error reports.
The extensions implementation is a copy of [http crate](https://crates.io/crates/http)'s
[Extensions](https://docs.rs/http/0.2.8/http/struct.Extensions.html) implementation.

`ExtensionExt` allows adding data of any type to error reports. At the application level,
you can access this data with trait `Extension` with method `report.extension_ref::<T>()`.

```rust
use extension_eyre::{eyre::eyre, SectionExt, Section, eyre::Report};
use std::process::Command;
use tracing::instrument;

trait Output {
    fn output2(&mut self) -> Result<String, Report>;
}

impl Output for Command {
    #[instrument]
    fn output2(&mut self) -> Result<String, Report> {
        let output = self.output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(eyre!("cmd exited with non-zero status code"))
                .with_section(move || stdout.trim().to_string().header("Stdout:"))
                .with_section(move || stderr.trim().to_string().header("Stderr:"))
        } else {
            Ok(stdout.into())
        }
    }
}
```

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
