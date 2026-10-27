#include <Accelerate/Accelerate.h>
#include <calceph.h>
#include <mach-o/dyld.h>
#include <stdio.h>
#include <string.h>
int main(int argc,char**argv) {
 printf("integer_bytes %zu\ndouble_bytes %zu\nordinary_long_double_bytes %zu\n",sizeof(__CLPK_integer),sizeof(double),sizeof(long double));
 printf("calceph_version %s\n",CALCEPH_VERSION_STRING);
 for(unsigned i=0;i<_dyld_image_count();i++){const char*p=_dyld_get_image_name(i);if(strstr(p,"BLAS")||strstr(p,"LAPACK")||strstr(p,"Accelerate"))printf("image %s\n",p);}
 if(argc!=2)return 2;
 t_calcephbin*e=calceph_open(argv[1]);if(!e)return 3;
 double start,end,pv[6]={0};int continuous;char version[CALCEPH_MAX_CONSTANTVALUE];
 if(!calceph_gettimespan(e,&start,&end,&continuous))return 4;
 if(!calceph_getfileversion(e,version))return 5;
 printf("ephemeris %s\nspan %.17g %.17g %d\ntimescale %d\n",version,start,end,continuous,calceph_gettimescale(e));
 int ok=calceph_compute(e,2458000,0.125,16,0,pv);printf("target16 %d %.17g\n",ok,pv[0]);
 calceph_close(e);return ok?0:6;
}
