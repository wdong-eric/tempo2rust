/* Same inputs and complete bin projection as central-ag-contraction-review.py. */
#include <cstdio>
#include <cstdint>
#include <cstring>
#include <vector>
extern void al_lomb(double*,double*,int,double,double,double*,double*,int*,double*);
static void emit(int test,const char*kind,int i,double value) {
    uint64_t bits;std::memcpy(&bits,&value,8);
    std::printf("%d\t%s\t%d\t%016llx\t%.17g\n",test,kind,i,(unsigned long long)bits,value);
}
int main() {
    for(int test=0;test<2;test++) {
        int n=test?17:16;std::vector<double>x(n),y(n),ox(n),oy(n);
        for(int i=0;i<n;i++) {
            x[i]=test?(20*i+i%3)/16.0:i*1.25;y[i]=((i*i+3*i)%11)*1e-6;
            emit(test,"x",i,x[i]);emit(test,"y",i,y[i]);
        }
        int count=0;double variance=0;
        al_lomb(x.data(),y.data(),n,1,1,ox.data(),oy.data(),&count,&variance);
        emit(test,"count",0,count);emit(test,"variance",0,variance);
        for(int i=0;i<count;i++) {emit(test,"frequency",i,ox[i]);emit(test,"power",i,oy[i]);}
    }
}
