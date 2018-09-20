# USB automator

This little utility can run commands when a device is plugged or unplugged.

The main use case is to change keyboard layout according to device presence.
It can also be used to start a backup when your external drive is plugged.

## Configuration

On first run a default config file will be created in `$XDG_CONFIG_HOME/usbautomator/config.toml`.
Configuration follows this pattern:

```toml
[devices.NAME]
on_plugged = "COMMAND"
on_unplugged = "COMMAND"
[devices.NAME.properties]
ID_VENDOR_ID = "feed"
ID_MODEL_ID = "1307"
ID_INPUT_KEYBOARD = "1"
KEY = "1000000000007 ff9f207ac14057ff febeffdfffefffff fffffffffffffffe"
```

### How to configure an automation ?

Let's say you want to switch layout when your keyboard is plugged.
We will configure usbautomator step by step.

First, add a section with your commands:

```toml
[devices.mykeyboard]
on_plugged = "setxkbmap us"
on_unplugged = "setxkbmap <your laptop layout or whatever>"
```

You then need to find some properties which uniquely identify your device.
**Warning**: a single physical device might show up as several devices on your operating system.

You can use the *watch* command:

    usbautomator -w

This will listen for changes and display the properties of the devices (there might be a lot of them).
Simply plug your keyboard to list changed devices properties.
The command can be killed as usual with `<ctrl-c>`.

You should see this kind of properties:

```
ID_FOR_SEAT: input-pci-0000_00_14_0-usb-0_1_1_3
ID_INPUT: 1
ID_INPUT_KEY: 1
ID_INPUT_KEYBOARD: 1
ID_MODEL: ErgoDox_EZ
[...]
ID_MODEL_ID: 1307
ID_VENDOR: ErgoDox_EZ
ID_VENDOR_ID: feed
KEY: 1000000000007 ff9f007ac04007ff febeffdfffefffff fffffffffffffffe
NAME: "ErgoDox EZ ErgoDox EZ"
PRODUCT: 3/feed/1307/111
```

Some properties depend on the USB bus, those are not useful to identify your device as they might change depending on the USB plug used to connect your device.
The `KEY` property seems a good fit to identify your device, let's add it to the configuration:

```toml
[devices.mykeyboard.properties]
KEY = "1000000000007 ff9f007ac04007ff febeffdfffefffff fffffffffffffffe"
```

Your keyboard layout should now change according to the device presence.

**Note**: if the properties used in the configuration match multiple devices, the on_plugged/on_unplugged commands will run multiple time.

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
