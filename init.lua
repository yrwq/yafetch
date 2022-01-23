-- [ check <https://ari-web.netlify.app/dotfile/gentoo/dotfiles/config/yafetch/init.lua> for an example user config ] --

local red = "\27[31m"
local grn = "\27[32m"
local yel = "\27[33m"
local blu = "\27[34m"
local wht = "\27[37m"
local bold = "\27[1m"
local res = "\27[0m"

local user_clr = res


yafetch.sep = " ~ "
yafetch.sep_color = bold

-- if set to false, yafetch.shell() will return
-- the full path of the current shell
yafetch.shell_base = true


local shell = yafetch.shell()
local shell_icon = " "

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


function running_as_root()
    local fd = io.open"/root"
    if fd == nil then return false else io.close(fd) return true end
end


if running_as_root() then
    user_clr = red .. bold
end

local ascii1 = wht .. "  ,d88b.d88b,    " .. user_clr
local ascii2 = red .. "  88888888888    " .. res
local ascii3 = grn .. "  `Y8888888Y´    " .. res
local ascii4 = yel .. "    `Y888Y´      " .. res
local ascii5 = blu .. "      `Y´        " .. res


yafetch.header_sep = string.format("%s@%s", blu, res)
yafetch.header_sep_color = wht
yafetch.header_format = ascii1  -- could be ascii1, an icon, etc.


function yafetch.init()
    yafetch.header()
    yafetch.format(ascii2 .. res .. red, distro_icon, wht, distro)
    yafetch.format(ascii3 .. res .. grn, shell_icon, wht, shell)
    yafetch.format(ascii4 .. res .. yel, kernel_icon, wht, kernel)
    yafetch.format(ascii5 .. res .. blu, pkgs_icon, wht, pkgs)
end
