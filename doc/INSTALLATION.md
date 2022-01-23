# Manual

_This example uses the https://github.com/TruncatedDinosour/yafetch, if you want the official just change out the git clone link_

- Clone repo

```bash
git clone https://github.com/TruncatedDinosour/yafetch && cd yafetch
```

- Configure it

```bash
chmod a+rx ./configure
```

Now configure the project to generate a makefile
You can see all options by passing `--help` flag to `./configure`

For example

```bash
./configure --use-clang --use-harden --use-strip --use-pedantic --use-warnings --use-werror --use-lto --use-config --use-extreme-strip --use-install
```

Will

```
- Use the `clang` compiler
- Harden the binary at compile time
- Strip the binary
- Use pedantic flags
- Detect more warnings
- Error on any warnings
- Enable LTO
- Enable configuration placement
- Make strip flags extreme
- Enable installation and uninstallation to be automatic
```

- Compile it

```bash
make -j8
```

- Install it (`--use-install`)

```bash
sudo make install
```

- Copy default configuration

```bash
mkdir -p ~/.config/yafetch

cp /usr/share/yafetch/init.lua ~/.config/yafetch
```

Full install:

```bash
git clone https://github.com/TruncatedDinosour/yafetch && cd yafetch

chmod a+rx ./configure

./configure --use-clang --use-harden --use-strip --use-pedantic --use-warnings --use-werror --use-lto --use-config --use-extreme-strip --use-install

make -j8

sudo make install

mkdir -p ~/.config/yafetch

cp /usr/share/yafetch/init.lua ~/.config/yafetch
```

**To recompile remember to `make clean` before the compilation step.**

To uninstall it just configure the makefile with `--use-install` and

```bash
make uninstall
```

# Arch linux (AUR)

https://aur.archlinux.org/packages/yafetch-git/ (NOTE: This is **not the fork, it's the original** and might be broken)

# Gentoo ebuild

[app-misc/yafetch::dinolay](https://ari-web.xyz/gentooatom/app-misc/yafetch)

## Suggestions, maintainers and more sources are welcome (PRs are open)
