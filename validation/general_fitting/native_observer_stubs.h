// Component probes link the public observer objects without exporting a fit trace.
int ae_solver_owner=-2;
void ae_svd(int,int,int,const longdouble*,double){}
void ae_update(pulsar*,int,int,int,const char*){}
void ae_prepared(int,pulsar*,const double*,const double*,const double*,const int*,int,double**,double**,double*,double**,double**){}
void ae_covariance(int,double**){}
void ae_solver_svd(int,const longdouble*){}
void ae_solver_details(int,int,int,const double*,bool,bool,double,const double*,bool,const double*,double**){}
void ae_solver_multiplier(double){}
void ae_robust_step(int,int,double,double,double,double,double*,double**,double*,double*,double*,double**,int,int){}
