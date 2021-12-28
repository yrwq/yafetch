#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <lua.h>

extern lua_State * L;

#define reset "\x1B[0m"
#define red "\x1B[31m"

void script_init(void);
void script_run(char * filename);
void script_destroy(void);
void func_reg(void);
char * config_location(void);
