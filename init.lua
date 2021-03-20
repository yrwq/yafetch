local red = "\27[31m"
local grn = "\27[32m"
local yel = "\27[33m"
local blu = "\27[34m"
local mag = "\27[35m"
local cyn = "\27[36m"
local wht = "\27[37m"
local bold = "\27[1m"
local res = "\27[0m"

yafetch.sep = " ~ "
yafetch.sep_color = bold

-- if set to false, yafetch.shell() will return
-- the full path of the current shell
yafetch.shell_base = true
local shell = yafetch.shell()
local shell_icon = " "

local username = yafetch.user()
local hostname = yafetch.hostname()

local kernel = yafetch.kernel()
local kernel_icon = " "

local pkgs = yafetch.pkgs()
local pkgs_icon = " "

local distro = yafetch.distro()
local distro_icon

if distro == "Arch Linux" then
    distro_icon = " "
elseif distro == "NixOS" then
    distro_icon = " "
elseif distro == "Ubuntu" then
    distro_icon = " "
elseif distro == "Alpine Linux" then
    distro_icon = " "
else
    distro_icon = " "
end

local ascii1 = wht .. "  ,d88b.d88b,    " .. res
local ascii2 = red .. "  88888888888    " .. res
local ascii3 = grn .. "  `Y8888888Y´    " .. res
local ascii4 = yel .. "    `Y888Y´      " .. res
local ascii5 = blu .. "      `Y´        " .. res

yafetch.header_sep = "@"
yafetch.header_sep_color = bold
yafetch.header_format = ascii1 -- could be ascii1, an icon, etc.

function yafetch.init()
    print("")
    yafetch.header()
    yafetch.format(ascii2 .. res .. red, distro_icon, wht, distro)
    yafetch.format(ascii3 .. res .. grn, shell_icon, wht, shell)
    yafetch.format(ascii4 .. res .. yel, kernel_icon, wht, kernel)
    yafetch.format(ascii5 .. res .. blu, pkgs_icon, wht, pkgs)
    print("")
end
