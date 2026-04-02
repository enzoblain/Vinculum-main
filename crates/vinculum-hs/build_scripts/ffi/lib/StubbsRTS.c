#include <stdlib.h>

extern void hs_init(int *argc, char ***argv);
extern void hs_exit(void);

static char *default_argv_array[] = {"vinculum", NULL};

void haskell_init(int argc, char **argv) {
    int init_argc = (argc > 0) ? argc : 1;
    char **init_argv = (argv != NULL) ? argv : default_argv_array;

    hs_init(&init_argc, &init_argv);
}

void haskell_exit(void) {
    hs_exit();
}
