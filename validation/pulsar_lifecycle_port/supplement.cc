// Defined caller witnesses around unchanged pinned source; never reads an unseeded link.
#include "tempo2.h"
#include "TKmatrix.h"
#include "t2fit_nestlike.h"
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <climits>
int main(int argc,char**argv){
 if(argc!=4)return 2;
 FILE*in=fopen(argv[1],"rb");FILE*out=fopen(argv[2],"wb");if(!in||!out)return 3;
 fprintf(out,"{\"nonce\":\"%s\",\"helpers\":[",argv[3]);
 unsigned count;fscanf(in,"%u",&count);
 for(unsigned i=0;i<count;i++){unsigned long long packed;int r,d,c;fscanf(in,"%llu %d %d %d",&packed,&r,&d,&c);if(i)fputc(',',out);
 fprintf(out,"{\"flags\":%llu,\"red\":%d,\"dm\":%d,\"chrom\":%d,\"encode\":%llu,\"set_red\":%llu,\"set_dm\":%llu,\"set_chrom\":%llu}",(unsigned long long)t2DecodeTNsubtractPolyFlags(packed),t2DecodeTNRedSubPolyOrd(packed),t2DecodeTNDMSubPolyOrd(packed),t2DecodeTNChromSubPolyOrd(packed),(unsigned long long)t2EncodeTNsubtractPoly(packed,r,d,c),(unsigned long long)t2SetTNRedSubPolyOrd(packed,r),(unsigned long long)t2SetTNDMSubPolyOrd(packed,d),(unsigned long long)t2SetTNChromSubPolyOrd(packed,c));}
 fputs("],\"shifts\":[",out);fscanf(in,"%u",&count);
 for(unsigned i=0;i<count;i++){unsigned long long p;int shift;fscanf(in,"%llu %d",&p,&shift);if(i)fputc(',',out);fprintf(out,"%d",t2DecodeTNSubPolyOrdStored(p,shift));}
 fputs("],\"clamps\":[",out);fscanf(in,"%u",&count);
 for(unsigned i=0;i<count;i++){int v;fscanf(in,"%d",&v);if(i)fputc(',',out);fprintf(out,"%d",t2ClampSubPolyOrder(v));}
 MAX_OBSN=4;MAX_PSR=1;debugFlag=0;displayCVSversion=0;
 pulsar*p=new pulsar;p->param[0].linkTo[2]=123;p->param[MAX_PARAMS-1].linkFrom[4]=-9;
 allocateMemory(p,0);fprintf(out,"],\"partial_links_allocate\":[%d,%d]",p->param[0].linkTo[2],p->param[MAX_PARAMS-1].linkFrom[4]);
 pulsar*old=new pulsar;std::memcpy(old->param,p->param,sizeof p->param);
 initialiseOne(p,1,1);fprintf(out,",\"partial_links_initialise\":[%d,%d],\"initial\":%llu}\n",p->param[0].linkTo[2],p->param[MAX_PARAMS-1].linkFrom[4],(unsigned long long)p->TNsubtractPoly);
 destroyMemory(old);destroyOne(p);delete old;delete p;fclose(in);fclose(out);return 0;
}
