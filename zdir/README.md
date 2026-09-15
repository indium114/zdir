# zdir

![preview of zdir's tui directory selector](assets/preview.png)

_zdir_ (pronounced 'zee-dir', like 'cedar') is a blazingly-faster `cd` alternative, inspired by [zoxide](https://github.com/ajeetdsouza/zoxide).

It uses a background daemon and socket instead of running everything synchronously when you `z` or `cd` into a directory, offering blazingly-fast speeds even on resource-constrained computers.

_zdir_ also features a TUI picker when a query has multiple results with a similar 'frecency' score (see the above image).

## benchmarks

> as of commit `212325d1`, using the `bench.nu` script on **nushell 0.115.1**

### zoxide benchmark

| metric | time             |
| ------ | ---------------- |
| mean   | 8ms 798µs 619ns  |
| min    | 5ms 768µs 335ns  |
| max    | 18ms 768µs 812ns |
| std    | 1ms 826µs 13ns   |

### zdir benchmark

| metric | time             |
| ------ | ---------------- |
| mean   | 4ms 762µs 404ns  |
| min    | 3ms 246µs 124ns  |
| max    | 12ms 194µs 279ns |
| std    | 1ms 649µs 303ns  |

## setup

### with home-manager

Add the repo to your flake inputs...

```nix
{
  inputs = {
    # ...
    zdir = {
      url = "github:indium114/zdir";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
}
```

And configure it in home-manager...

```nix
{
  inputs,
  ...
}:

{

  programs.zdir = {
    enable = true; # installs the zdir binary and sets up the systemd user service
    enableNushellIntegration = true; # adds the integration script to nushell
  };

}
```

### setting up zdir-librarian, the background daemon

#### as a systemd user service

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

### setting up shell integration

> [!note]
> Currently, there is only shell integration for [nushell](https://nushell.sh). \
> If you are proficient in another shell's scripting language, feel free to write shell integration for that shell and open a pull request.

> [!warning]
> When running with a new database, make sure that the first path you give is a full path. This will allow `zdir-librarian` to create the table in the database. If you give a nonexistent path the first time, the librarian will panic.

#### nushell

1. Download the `shell/zdir.nu` file in this repo.
2. Save it so somewhere like `~/.config/nushell/zdir.nu`
3. In your `config.nu` file (open it by running `config nu`), add the following line:

```nu
source ~/.config/nushell/zdir.nu
```

Now, assuming the `zdir` executable is on `$env.PATH`, it'll work!
