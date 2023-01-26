# Local Plugin
Local plugin for Done

# Dependencies
- libsqlite3

## Ubuntu
```
sudo apt install -y libsqlite3-dev
```

## Fedora
```
sudo dnf install -y sqlite-devel
```

## Arch Linux
```
sudo pacman -S sqlite-devel --noconfirm
```

# Release
```
cargo build --release
```
