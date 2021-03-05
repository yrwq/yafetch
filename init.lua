local red = "\27[31m"
local grn = "\27[32m"
local yel = "\27[33m"
local blu = "\27[34m"
local mag = "\27[35m"
local cyn = "\27[36m"
local wht = "\27[37m"
local bold = "\27[1m"

yafetch.sep = " ~ "
yafetch.sep_color = bold

yafetch.header_sep = "@"
yafetch.header_sep_color = bold

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
elseif distro == "Ubuntu" then
    distro_icon = " "
elseif distro == "Alpine Linux" then
    distro_icon = " "
else
    distro_icon = " "
end

function yafetch.init()
    print("")
    yafetch.header(wht, hostname, wht, username)
    yafetch.format(red, distro_icon, wht, distro)
    yafetch.format(grn, shell_icon, wht, shell)
    yafetch.format(yel, kernel_icon, wht, kernel)
    yafetch.format(blu, pkgs_icon, wht, pkgs)
    print("")
end
