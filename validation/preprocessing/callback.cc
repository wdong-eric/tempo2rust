// Declared external select-plugin input; loaded by the unchanged core dlopen/dlsym.
#include "tempo2.h"
extern "C" {
#ifndef OMIT_VERSION
const char *plugVersionCheck =
#ifdef WRONG_VERSION
"fixture-wrong-version";
#else
TEMPO2_h_VER;
#endif
#endif
#ifndef OMIT_ENTRY
char *selectInterface(int argc,char **argv,pulsar *psr,int *npsr) {
    static int calls=0;
    ++calls;
    for(int p=0;p<*npsr;p++) {
        psr[p].nits += calls;
        if(psr[p].nobs>0)psr[p].obsn[0].toaErr += argc;
    }
    printf("fixture callback call=%d argc=%d npsr=%d argv0=%s\n",calls,argc,*npsr,argv[0]);
    return nullptr;
}
#endif
}
