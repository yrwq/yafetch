local red = "\27[31m"
local grn = "\27[32m"
local yel = "\27[33m"
local blu = "\27[34m"
local mag = "\27[35m"
local wht = "\27[37m"
local bold = "\27[1m"
local res = "\27[0m"

local header = bold .. mag .. yafetch.user() .. res .. bold .. "@" .. bold .. mag .. yafetch.hostname() .. res

local used_ssd = yafetch.disk_total("/") - yafetch.disk_free("/")

print(header)
print(yel .. "distro    " .. res .. bold .. yafetch.os() .. res)
print(yel .. "host      " .. res .. bold .. yafetch.host() .. res)
print(yel .. "uptime    " .. res .. bold .. yafetch.uptime() .. res)
print(yel .. "kernel    " .. res .. bold .. yafetch.kernel() .. res)
print(yel .. "cpu       " .. res .. bold .. yafetch.cpu() .. res)
print(yel .. "disk /    " .. res .. bold .. used_ssd .. "gb  / " .. yafetch.disk_total("/") .. "gb" .. res)
print(yel .. "memory    " .. res .. bold .. yafetch.mem_used() .. "gb / " .. yafetch.mem_total() .. "gb" .. res)
