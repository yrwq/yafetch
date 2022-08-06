init = function()
    yafetch.header()
    yafetch.format("distro", yafetch.distro())
    yafetch.format("music", yafetch.music())
end
