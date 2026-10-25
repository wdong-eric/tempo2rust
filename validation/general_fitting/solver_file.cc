// Shared binary64 inputs, independently optional source pointers and histories.
#include "TKfit.h"
#include "TKmatrix.h"
#include "TKlog.h"
#include <cstdio>
#include <cstdlib>
static FILE* out;
void ae_solver_svd(int n, const longdouble* w) {
    for(int i=0;i<n;i++) { char value[128]; quadmath_snprintf(value,sizeof value,"%.36Qg",w[i]);fprintf(out,"S %d %s\n",i,value); }
}
void ae_robust_step(int count,int final,double sigma,double next,double phi,double dphi,
        double*y,double**m,double*r,double*p,double*e,double**c,int n,int nf) {
    fprintf(out,"R %d %d %.17g %.17g %.17g %.17g\n",count,final,sigma,next,phi,dphi);
    for(int i=0;i<n;i++) {
        fprintf(out,"Y %d %.17g %.17g\n",i,y[i],r[i]);
        for(int j=0;j<nf;j++)fprintf(out,"A %d %d %.17g\n",i,j,m[i][j]);
    }
    for(int j=0;j<nf;j++) {
        fprintf(out,"I %d %.17g %.17g\n",j,p[j],e[j]);
        for(int k=0;k<nf;k++)fprintf(out,"K %d %d %.17g\n",j,k,c[j][k]);
    }
}
int main(int argc,char**argv) {
    if(argc<3)return 2;
    FILE*in=fopen(argv[1],"r");out=fopen(argv[2],"w");
    int n,np,nc,rescale;
    if(!in||!out||fscanf(in,"%d %d %d %d",&n,&np,&nc,&rescale)!=4)return 3;
    int mask=argc>3?atoi(argv[3]):7;
    bool data=argc>4?atoi(argv[4]):true;
    char robust=argc>5?argv[5][0]:0;
    writeResiduals=argc>6?atoi(argv[6]):0;
    debugFlag=argc>7?atoi(argv[7]):0;
    double**a=malloc_blas(n,np),**w=malloc_blas(n,np),**c=nc?malloc_blas(nc,np):nullptr,**cov=malloc_blas(np,np);
    double*y=new double[n],*wy=new double[n],*targets=new double[nc],*p=new double[np],*e=new double[np];
    for(int i=0;i<n;i++) {
        if(fscanf(in,"%lg %lg",y+i,wy+i)!=2)return 4;
        for(int j=0;j<np;j++)if(fscanf(in,"%lg %lg",a[i]+j,w[i]+j)!=2)return 4;
    }
    for(int i=0;i<nc;i++) {
        if(fscanf(in,"%lg",targets+i)!=1)return 4;
        for(int j=0;j<np;j++)if(fscanf(in,"%lg",c[i]+j)!=1)return 4;
    }
    for(int j=0;j<np;j++)p[j]=e[j]=-9876;
    double chi=TKrobustDefConstrainedLeastSquares(data?y:nullptr,data?wy:nullptr,a,w,c,n,np,nc,1e-27,rescale,mask&1?p:nullptr,mask&2?e:nullptr,mask&4?cov:nullptr,robust,targets);
    fprintf(out,"CHI %.17g\n",chi);
    for(int j=0;j<np;j++) {
        if(mask&1)fprintf(out,"P %d %.17g\n",j,p[j]);
        if(mask&2)fprintf(out,"E %d %.17g\n",j,e[j]);
        if(mask&4)for(int k=0;k<np;k++)fprintf(out,"C %d %d %.17g\n",j,k,cov[j][k]);
    }
    fclose(out);fclose(in);
    free_blas(a);free_blas(w);if(c)free_blas(c);free_blas(cov);
    delete[]y;delete[]wy;delete[]targets;delete[]p;delete[]e;
}
