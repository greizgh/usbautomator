# USB automator

This little utility can run commands when a device is plugged or unplugged.

The main use case is to change keyboard layout according to device presence.

## Configuration

On first run a default config file will be created in `$XDG_CONFIG_HOME/usbautomator/config.toml`.
Configuration follows this pattern:

```toml
[devices.NAME]
on_plugged = "COMMAND"
on_unplugged = "COMMAND"
[devices.NAME.properties]
product = "USB PRODUCT ID"
```

## Building

You will need [rust](https://www.rust-lang.org) and [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Then simply run:

    cargo build --release

## Packages

[Archlinux](https://aur.archlinux.org/packages/usbautomator)

## Changelog

See [CHANGELOG.md](./CHANGELOG.md)

## License

GPLv3
