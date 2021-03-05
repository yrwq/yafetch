local red = "\27[31m"
local grn = "\27[32m"
local yel = "\27[33m"
local blu = "\27[34m"
local mag = "\27[35m"
local cyn = "\27[36m"
local wht = "\27[37m"
local bold = "\27[1m"

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

function yafetch.init()
    print("")
    yafetch.header(red, hostname, wht .. bold, username)
    yafetch.format(grn, shell_icon, wht .. bold, shell)
    yafetch.format(blu, kernel_icon, wht .. bold, kernel)
    yafetch.format(mag, pkgs_icon, wht .. bold, pkgs)
    print("")
end
