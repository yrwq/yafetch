# Manual
```bash
git clone https://github.com/TruncatedDinosour/yafetch && cd yafetch
make -j8
make CONF="${HOME}/.config/yafetch" config  # optional (copies default config to $HOME/.config/yafetch)
make strip -j8  # optional, strips the binary
sudo make install
```

# Arch linux (AUR)
https://aur.archlinux.org/packages/yafetch-git/  (NOTE: This is **not the fork, it's the original**)

# Gentoo ebuild
[app-misc/yafetch::dinolay](https://ari-web.netlify.app/gentooatom/app-misc/yafetch)

## Suggestions and more sources are welcome (PRs are open)
