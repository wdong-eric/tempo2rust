/* Actual simultaneous callers obey AJ/AK process-global C exclusivity. The
 * mutex covers borrowed streams, callbacks, names/segments and global reads. */
#include "tempo2pred.h"
#include "tempo2pred_int.h"
#include <assert.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
extern int ak_consumer_main(int,char**);
extern int ak_storage_main(int,char**);
extern int ak_low_count_main(void);
extern int ak_boundary_main(void);
static pthread_mutex_t caller_exclusive=PTHREAD_MUTEX_INITIALIZER;
static void* caller(void* argument) {
    intptr_t id=(intptr_t)argument;
    for(int pass=0;pass<3;pass++) {
        assert(pthread_mutex_lock(&caller_exclusive)==0);
        printf("AK_BEGIN %ld %d\n",(long)id,pass);
        tempo2_verbose=1;
        ChebyModelSet_OutOfRange=0;
        char storage[64];
        snprintf(storage,sizeof(storage),"storage-%ld-%d.tsv",(long)id,pass);
        char *argv[]={(char*)"ak-caller",storage,NULL};
        assert(ak_consumer_main(1,argv)==0);
        assert(ak_storage_main(2,argv)==0);
        assert(ak_low_count_main()==0);
        assert(ak_boundary_main()==0);
        printf("AK_END %ld %d\n",(long)id,pass);
        assert(pthread_mutex_unlock(&caller_exclusive)==0);
    }
    return NULL;
}
int main(void) {
    alarm(90); /* bounded deadlock detection, includes synchronous callbacks */
    pthread_t threads[4];
    for(intptr_t i=0;i<4;i++) assert(pthread_create(&threads[i],NULL,caller,(void*)i)==0);
    for(int i=0;i<4;i++) assert(pthread_join(threads[i],NULL)==0);
    puts("AK four caller threads completed twelve exclusive full workflows");
    return 0;
}
