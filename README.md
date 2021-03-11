<p align="center"> <img src="http://0x0.st/-P91.png"> </p>

<h4 align="center">Yafetch is a minimal command line system information tool written in C and configured in Lua. </h4>

# Dependencies

- a linux distribution
- a compiler
- `lua5.1`

# Installation

```zsh
git clone https://github.com/yrwq/yafetch && cd yafetch
make
make config # optional (copies default config to $HOME/.config/yafetch)
sudo make install
```

# Usage

`yafetch` or `yafetch <config.lua>`

# Configuration

Yafetch is extensible in lua, the default location for the config is `~/.config/yafetch/init.lua`

[variables](https://github.com/yrwq/yafetch#variables)
[functions](https://github.com/yrwq/yafetch#functions)

## Variables

### yafetch.sep

Separator used between the information and the icon.
Can be any string or character.

```lua
yafetch.sep = " ~ "
```

### yafetch.sep_color

Foreground color used to colorize the separator.

```lua
yafetch.sep_color = "\27[31m"
```

### yafetch.header_sep

The [separator](yafetch.sep) used in the header.

### yafetch.header_sep_color

The [separator color](yafetch.sep_color) used in the header.

## Functions

### yafetch.init()

Main function of the fetch, formatting goes in this function.

```lua
yafetch.init = function()
    -- formatting
end
```

### yafetch.format()

Format a line in the fetch, takes 4 arguments,

```lua
yafetch.format(color, identifier, color2, information)
```

The colors should escape sequences,

The identifier can be any string, personally i use icons from [Nerd Fonts](https://github.com/ryanoasis/nerd-fonts)

The information should be a function from below.

### yafetch.header()

Format a header, usually your username and hostname goes here.

```lua
yafetch.header(color, yafetch.hostname(), color, yafetch.username())
```

### yafetch.distro()

Returns current linux distrobution

### yafetch.pkgs()

Detects package manager, returns number of packages installed.

### yafetch.kernel()

Returns kernel version

### yafetch.user()

Returns username

### yafetch.hostname()

Returns hostname

### yafetch.shell()

Returns shell

