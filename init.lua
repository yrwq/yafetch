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
local pkgs_icon = " "

local distro = yafetch.distro()
local distro_icon

local cpu = yafetch.cpu()
local cpu_icon = " "

local local_ip = yafetch.local_ip()
local local_ip_icon = "󰲝 "

local public_ip = yafetch.public_ip()
local public_ip_icon = "󰛳 "

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

yafetch.header_sep = string.format("%s@%s", blu, res)
yafetch.header_sep_color = wht
yafetch.header_format = " " .. yafetch.sep -- could be ascii1, an icon, etc.

function yafetch.init()
    yafetch.header()
    yafetch.format(res .. red, distro_icon, wht, distro)
    yafetch.format(res .. grn, shell_icon, wht, shell)
    yafetch.format(res .. yel, kernel_icon, wht, kernel)
    yafetch.format(res .. yel, local_ip_icon, wht, local_ip)
    yafetch.format(res .. yel, public_ip_icon, wht, public_ip)
    yafetch.format(res .. yel, cpu_icon, wht, cpu)
    yafetch.format(res .. blu, pkgs_icon, wht, pkgs)
end
