// Observers only: source files retain their own FILE* lifetime/cursor semantics.
static void wire_generate(const char*prefix){
 longdouble v[]={0,-0.0Q,1,-2.5Q,scalbnq(1,-16494),scalbnq(1,-16382)-scalbnq(1,-16494),scalbnq(1,-16382),FLT128_MAX,HUGE_VALQ,-HUGE_VALQ,nanq(""),-nanq(""),0.1Q,-0.1Q,1e-30Q,1e30Q,3};
 for(int kind=0;kind<2;kind++){std::string path=std::string(prefix)+(kind?".general.bin":".tensor.bin");FILE*f=fopen(path.c_str(),"wb");gwSrc t[2]={};gwgeneralSrc g[2]={};for(int i=0;i<2;i++)for(int j=0;j<(kind?17:9);j++)if(kind)((longdouble*)&g[i])[j]=v[(j+i*9)%17];else ((longdouble*)&t[i])[j]=v[(j+i*9)%17];for(int id:{2,9})if(kind)GWgeneralbackground_write(g,f,2,id);else GWbackground_write(t,f,2,id);fclose(f);}
}
static void wire(const char*input,const char*output){
 for(int kind=0;kind<2;kind++){std::string name=kind?"general":"tensor",a=std::string(input)+"."+name+".bin",b=std::string(output)+"."+name+".bin";FILE*f=fopen(a.c_str(),"rb"),*o=fopen(b.c_str(),"wb");if(!f||!o)exit(4);for(int id:{2,9}){rewind(f);gwSrc t[100]={};gwgeneralSrc g[100]={};int n=kind?GWgeneralbackground_read(g,f,id):GWbackground_read(t,f,id);emit(name+"."+std::to_string(id)+".count","exact",n);emit(name+"."+std::to_string(id)+".cursor","exact",ftell(f));for(int i=0;i<n;i++)for(int j=0;j<(kind?17:9);j++){unsigned __int128 bits;memcpy(&bits,&(kind?((longdouble*)&g[i]):((longdouble*)&t[i]))[j],16);std::string s;do{s+=char('0'+bits%10);bits/=10;}while(bits);std::reverse(s.begin(),s.end());text(name+"."+std::to_string(id)+"."+std::to_string(i)+"."+std::to_string(j),"bits",s.c_str());}if(kind)GWgeneralbackground_write(g,o,n,id);else GWbackground_write(t,o,n,id);}fclose(f);fclose(o);}
}
static void eccentric(pulsar*ps,const char*control){
 FILE*f=fopen(control,"r");if(!f)exit(4);readSimpleParfile(f,ps);fclose(f);int flag=0;double period=ps->gwecc_orbital_period*365.25*86400.,e=ps->gwecc_e,epoch=ps->gwecc_epoch,theta=ps->gwecc_theta_0,a=pow(6.67e-11*(ps->gwecc_m1+ps->gwecc_m2)*1.9891e30*pow(2*M_PI/period,-2.),1./3.);int call=0;
 for(int i:{0,1,0,2}){int flag2=flag;double p2=period,e2=e,a2=a,epoch2=epoch,t2=theta;float energy;auto r=eccResWithEnergy(ps,i,&flag,&period,&e,&a,&epoch,&theta,&energy),plain=eccRes(ps,i,&flag2,&p2,&e2,&a2,&epoch2,&t2);std::string key="ecc."+std::to_string(call++);emit(key+".residual","seconds",r/1e9);emit(key+".plain","seconds",plain/1e9);emit(key+".energy","f32",energy);emit(key+".flag","exact",flag);const char*names[]={"period","ecc","a","epoch","theta"};double values[]={period,e,a,epoch,theta};for(int j=0;j<5;j++)emit(key+"."+names[j],"history",values[j]);}
 flag=1;float energy;emit("coalesced.residual","seconds",eccResWithEnergy(ps,0,&flag,&period,&e,&a,&epoch,&theta,&energy));emit("coalesced.energy","f32",energy);emit("Rs","history",Rs(1e30));emit("dadt","history",dadt(.2,1e12,1e30,2e30));emit("dedt","history",dedt(.2,1e12,1e30,2e30));emit("dtdt","history",dtdt(.2,.3,1e8));emit("Fe","history",Fe(.2));
}
extern double data[90000];
static void diagnostics(double sentinel){
 const int n=8;double x[n],y[n],sx[n],sy[n];for(int i=0;i<n;i++){x[i]=i*1.25;y[i]=((i*i+3*i)%11)*1e-6;data[i]=sentinel;}
 double**L=malloc_blas(n,n);for(int i=0;i<n;i++)for(int j=0;j<n;j++)L[i][j]=i==j?1e-6:0;writeResiduals=7;calcSpectra(L,x,y,n,sx,sy,2);vector("diagnostic.frequency","frequency_day",sx,2);vector("diagnostic.power","yr^3",sy,2);free_blas(L);
}
static void bugs(){
 double x0[]={0,.3,1.1,2.7,4.2,6,7.4,10},e0[]={1,2,1.5,.8,1.2,2.1,.9,1.4},y0[8],f[8],p[8],re[8],im[8];int n;double w=2*M_PI/(10.*8./7.);for(int i=0;i<8;i++)y0[i]=2*sin(w*x0[i])+3*cos(w*x0[i]);TK_weightLS(x0,y0,e0,8,f,p,&n,re,im);emit("elimination.cos","coefficient_s",re[0]);emit("elimination.sin","coefficient_s",im[0]);
 for(int reverse=0;reverse<2;reverse++){double x[8],y[8],e[8];for(int i=0;i<8;i++){int k=reverse?7-i:i;x[i]=x0[k];y[i]=y0[k];e[i]=e0[k];}TKspectrum(x,y,e,8,0,1,0,0,0,4,1,1,3,f,p,&n,0,0,re,im);vector(reverse?"sort.reversed":"sort.direct","s^2",p,n+1);}
 for(int sentinel:{0,7}){gwgeneralSrc g={};g.across_g=sentinel;int counts[]={0,0,0,1};long seed=-42;gwgenSpec spec={1e-15,1e-15,1e-15,1e-15,0,0,0,0};GWgeneralbackground(&g,counts,&seed,(double)1e-9,(double)1e-7,spec,0);const char*names[]={"plus","cross","st","sl"};longdouble values[]={g.aplus_g,g.across_g,g.ast_g,g.asl_g};for(int i=0;i<4;i++)emit("linear."+std::to_string(sentinel)+"."+names[i],"amplitude",values[i]);}
}
