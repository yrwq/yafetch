<p align="center"> <img src="http://0x0.st/-P91.png"> </p>

<h3 align="center">Yafetch is a minimal command line system information tool written in C and configured in Lua. </h3>

## Dependencies

- a linux distribution
- a compiler
- `lua5.1`

## Installation

```zsh
git clone https://github.com/yrwq/yafetch && cd yafetch
make
make config # optional (copies default config to $HOME/.config/yafetch)
sudo make install
```

## Usage

`yafetch` or `yafetch <config.lua>`

## Configuration

[variables](https://github.com/yrwq/yafetch#variables)
[functions](https://github.com/yrwq/yafetch#functions)

### Variables

#### yafetch.sep
#### yafetch.sep_color
#### yafetch.header_sep
#### yafetch.header_sep_color

### Functions

##### yafetch.init()
##### yafetch.format()
##### yafetch.header()
##### yafetch.distro()
##### yafetch.pkgs()
##### yafetch.kernel()
##### yafetch.user()
##### yafetch.hostname()
##### yafetch.shell()

