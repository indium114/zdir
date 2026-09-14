# zdir-librarian

_zdir-librarian_ is the background daemon for [zdir](https://github.com/indium114/zdir) ([crates.io](https://crates.io/crates/zdir))

## setup

### as a systemd user service

Write the following unit file as `~/.config/systemd/user/zdir-librarian.service`:

```systemd
[Unit]
Description=zdir-librarian

[Service]
ExecStart=zdir-librarian

[Install]
WantedBy=default.target
```

Then, enable and start the service.

```shell
systemctl enable --now --user zdir-librarian.service
```
