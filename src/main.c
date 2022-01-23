#include "yafetch.h"

int main(int argc, char **argv) {
    /* Create a lua vm */
    script_init();

    /* Register lua functions */
    func_reg();

    /* Load configuration file */
    if (argc != 2) {
        char *user_conf = config_location();
        char *config_file;

        if (access(user_conf, F_OK) == 0)
            config_file = user_conf;
        else
            config_file = "/usr/share/yafetch/init.lua";

        if (access(config_file, F_OK) != -1)
            script_run(config_file);
        else
            printf("\n%sYafetch%s: %s doesn't exist!\n\n", red, reset,
                   config_file);
    } else
        script_run(argv[1]);

    /* Destroy lua vm */
    script_destroy();

    return 0;
}
