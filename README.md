# cct

Simple command-line tool to copy file contents to clipboard on Linux.

## Install

### Arch

[Aur package](https://aur.archlinux.org/packages/copycontent) 

With yay
```bash
yay -S copycontent
```
With paru
```bash 
paru -S copycontent
```

### From Source
```bash
git clone https://github.com/reazndev/CopyContent
cd cct
cargo build --release
sudo cp target/release/cct /usr/local/bin/
```

Requires `wl-clipboard`:
```bash
sudo pacman -S wl-clipboard  # Arch
sudo apt install wl-clipboard  # Ubuntu/Debian
```

## Usage

```bash
cct myfile.txt
cct /path/to/config.json
cct ~/.bashrc
```

That's it. File goes to clipboard.

## Why?

Got tired of `cat file | wl-copy`. This is shorter.
