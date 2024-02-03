# Configuring Yafetch

## Variables

#### yafetch.shell_base

```lua
-- if set to true, yafetch.shell() will return only the currently
-- used shell's name,
-- example: zsh

-- if set to false, yafetch.shell() will return the currently 
-- used shell's full path,
-- example: /usr/bin/zsh

yafetch.shell_base = true -- false

```

> [yafetch.shell()](https://github.com/yrwq/yafetch/blob/main/docs.md#yafetch.shell())

#### yafetch.sep

```lua
-- separator used between the information and the identifier
-- can be any string, character, integer

yafetch.sep = " ~ "
```

#### yafetch.sep_color

Foreground color of yafetch.sep

```lua
yafetch.sep_color = "\27[31m" -- red

-- or

local red = "\27[31m"
yafetch.sep_color = red
```

> [yafetch.sep](https://github.com/yrwq/yafetch/blob/main/docs.md#yafetch.sep)

#### yafetch.header_sep

Same as [yafetch.sep](https://github.com/yrwq/yafetch/blob/main/docs.md#yafetch.sep), but in the header.

#### yafetch.header_sep_color

Same as [yafetch.sep_color](https://github.com/yrwq/yafetch/blob/main/docs.md#yafetch.sep_color), but in the header.

#### yafetch.header_format

Extra formatting to the header.

Can be any string, character, escape sequence.

```lua
yafetch.header_format = ascii[1]
yafetch.header_format = "\27[1m"
yafetch_header_format = "    "
```

## Functions

#### yafetch.init()

The main function of the fetch, formatting goes in this function

```lua
yafetch.init = function()
end

-- example:
yafetch.init = function()
	yafetch.format(red, "user", color2, yafetch.user())
end
```

> [yafetch.user()](https://github.com/yrwq/yafetch/blob/main/docs.md#yafetch.user())

#### yafetch.format()

Formats a line in the fetch, takes 4 arguments, optionally you can concat more.

```lua
local red = "\27[1m"
local reset = "\27[m"
yafetch.format(red, "user", reset, yafetch.user())
```
> [yafetch.user()](https://github.com/yrwq/yafetch/blob/main/docs.md#yafetch.user())

### Functions that returns information

##### yafetch.header()
##### yafetch.distro()
##### yafetch.kernel()
##### yafetch.local_ip()
##### yafetch.public_ip()
##### yafetch.cpu()
##### yafetch.pkgs()
##### yafetch.user()
##### yafetch.hostname()
##### yafetch.shell()
