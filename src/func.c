#include "yafetch.h"
#include <lauxlib.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/sysinfo.h>
#include <sys/utsname.h>
#include <sys/mount.h>
#include <sys/param.h>

#define LFUNC(N) int lua_##N(lua_State * L)

/* yafetch.user() */
/* Returns 'USER' environment variable */
LFUNC(user){

    char * username = getenv("USER");
    if(username) {
        lua_pushstring(L, username);
    } else lua_pushstring(L, "unknown");

    return 1;
}

LFUNC(distro) {
    char *def = malloc(512);
    char *new = malloc(512);
    int line = 0;
    FILE *f = fopen("/etc/os-release", "rt");

    while (fgets(def, 512, f)) {
        snprintf(new, 512, "%.*s", 511, def+4);
        if (strncmp(new, "=", 1) == 0) break;
        line++; }
    fclose(f);
    free(def);

    if (strncmp(new, "=", 1) == 0) {
        int len = strlen(new);
        for (int i = 0; i<len; i++){
            if (new[i] == '\"' || new[i] == '=') {
                for (int ii = 0; ii<len; ii++) new[ii] = new[ii+1];
                new[strlen(new)-1] = '\0';
            }
         }
    }
    lua_pushstring(L, new);
    return 1;
}

/* yafetch.hostname() */
/* Returns hostname */
LFUNC(hostname){
    char hostname[1024];
    hostname[1023] = '\0';
    gethostname(hostname, 1023);
    lua_pushstring(L, hostname);
    return 1;
}

/* yafetch.pkgs() */
/* Returns number of installed packages */
LFUNC(pkgs){
    int apt, dnf, emerge, flatpak, nix, pacman, rpm, xbps, total = 0;

    FILE *file[8];
    file[0] = popen("dpkg-query -f '${binary:Package}\n' -W 2> /dev/null | wc -l", "r");
    file[1] = popen("dnf list installed 2> /dev/null | wc -l", "r");
    file[2] = popen("qlist -I 2> /dev/null | wc -l", "r");
    file[3] = popen("flatpak list 2> /dev/null | wc -l", "r");
    file[4] = popen("nix-store -q --requisites /run/current-sys_vartem/sw 2> /dev/null | wc -l", "r");
    file[5] = popen("pacman -Qq 2> /dev/null | wc -l", "r");
    file[6] = popen("rpm -qa --last 2> /dev/null | wc -l", "r");
    file[7] = popen("xbps-query -l 2> /dev/null | wc -l", "r");

    fscanf(file[0], "%d", &apt);
    fscanf(file[1], "%d", &dnf);
    fscanf(file[2], "%d", &emerge);
    fscanf(file[3], "%d", &flatpak);
    fscanf(file[4], "%d", &nix);
    fscanf(file[5], "%d", &pacman);
    fscanf(file[6], "%d", &rpm);
    fscanf(file[7], "%d", &xbps);
    for (int i = 0; i < 8; i++) fclose(file[i]);

    if (apt > 0) total += apt;
    if (dnf > 0) total += dnf;
    if (emerge > 0) total += emerge;
    if (flatpak > 0) total += flatpak;
    if (nix > 0) total += nix;
    if (pacman > 0) total += pacman;
    if (rpm > 0) total += rpm;
    if (xbps > 0) total += xbps;

    lua_pushinteger(L, total);
    return 1;
}

/* yafetch.kernel() */
/* Returns kernel version */
LFUNC(kernel) {
    static char ret[255];
    struct utsname sys;

    uname(&sys);

    char * kernel = sys.release;
    lua_pushstring(L, kernel);
    return 1;
}

/* yafetch.shell() */
/* Returns path of shell */
LFUNC(shell){

    lua_getfield(L, LUA_GLOBALSINDEX, "yafetch");
    lua_getfield(L, -1, "shell_base");

    const int shell_full = lua_toboolean(L, -1);
    char * shell = getenv("SHELL");

    /* Get basename of shell by looking for last '/' */
    char * slash = strrchr(shell, '/');

    if (shell_full == 1) {
        if (slash) {
            shell = slash + 1;
        }
        lua_pushstring(L, shell);
    } else {
        lua_pushstring(L, shell);
    }

    return 1;
}

/* yafetch.header() */
LFUNC(header){

    lua_getfield(L, LUA_GLOBALSINDEX, "yafetch");

    lua_getfield(L, -1, "header_sep");
    const char * sep = lua_tostring(L, -1);

    lua_getfield(L, LUA_GLOBALSINDEX, "yafetch");

    lua_getfield(L, -1, "header_sep_color");
    const char * sep_color = lua_tostring(L, -1);
    lua_pop(L, 0);

    /* Get arguments from lua function */
    const char * h1_col = lua_tostring(L, 1);
    const char * h1 = lua_tostring(L, 2);

    /* Info */
    const char * h2_col = lua_tostring(L, 3);
    const char * h2 = lua_tostring(L, 4);

    printf("%7s%s%s%s%s%s%s%s%s\n",
            h1_col, h1, reset, sep_color, sep, reset, h2_col, h2, reset);
    return 1;
}

/* yafetch.format() */
/* Formats given strings. */
/* Helpers function to output information */
LFUNC(format){

    lua_getfield(L, LUA_GLOBALSINDEX, "yafetch");

    lua_getfield(L, -1, "sep");
    const char * sep = lua_tostring(L, -1);

    lua_getfield(L, LUA_GLOBALSINDEX, "yafetch");

    lua_getfield(L, -1, "sep_color");
    const char * sep_color = lua_tostring(L, -1);
    lua_pop(L, 0);

    /* Get arguments from lua function */
    /* Icon */
    const char * col_icon = lua_tostring(L, 1);
    const char * icon = lua_tostring(L, 2);

    /* Info */
    const char * col_info = lua_tostring(L, 3);
    const char * info = lua_tostring(L, 4);

    printf("%7s%s%s%s%s%s%s%s%s\n",
            col_icon, icon, reset, sep_color, sep, reset, col_info, info, reset);
    return 1;
}

/* Register functions in lua */
void func_reg(void){
#define REG(N) lua_pushcfunction(L, lua_##N); lua_setfield(L, -2, #N);
    lua_newtable(L);
    REG(shell)
    REG(user)
    REG(hostname)
    REG(distro)
    REG(kernel)
    REG(pkgs)
    REG(format)
    REG(header)
    lua_setfield(L, LUA_GLOBALSINDEX, "yafetch");

    luaL_newmetatable(L, "yafetch");
    lua_newtable(L);
    lua_setfield(L, -2, "__index");
    lua_pop(L, 1);
#undef REG
}
