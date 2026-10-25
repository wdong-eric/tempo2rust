"""Small source corrections with focused AE witnesses; observation hooks are separate."""
def corrected(name,source):
 if name=='TKfit.C':
  source=source.replace('augmented_white_data[i+ndata] = 0;','augmented_white_data[i+ndata] = constraint_vals ? constraint_vals[i] : 0;')
  begin=source.index('double TKrobustLeastSquares(');end=source.index('double TKrobustConstrainedLeastSquares(',begin)
  source=source[:begin]+source[begin:end].replace('outP, e,cvm,0);','outP, e,cvm,robust);')+source[end:]
 elif name=='t2fit_nestlike.C':
  source=source.replace('static std::map<std::tuple<int,int,param_label>, CoeffCache> cache;','std::map<std::tuple<int,int,param_label>, CoeffCache> cache;')
 elif name=='t2fit_stdFitFuncs.C':
  source=source.replace('dotproduct(psr->posPulsar,psr->obsn[ipos].planet_ssb[k])','dotproduct(psr[ipsr].posPulsar,psr[ipsr].obsn[ipos].planet_ssb[k])')
 elif name=='t2fit_fitwaves.C':
  source=source.replace('int k = kk % (psr[ipsr].nWhite*2);','int k = kk % ((label==param_wave_dm ? psr[ipsr].nWhite_dm : psr[ipsr].nWhite)*2);')
 elif name=='constraints_covar.C':
  source=source.replace('int idx=psr[ipsr].ifuncN;','int idx=k;')
 elif name=='bootstrap.C':
  source=source.replace('    longdouble result[MAX_PARAMS][MAX_ITER];','    auto result = new longdouble[MAX_PARAMS][MAX_ITER];')
  source=source.replace('    return 0;','    delete[] result;\n    return 0;',1)
 elif name=='choleskyAutomatic.C':
  # Six double buffers only reserve half their declared element capacity.
  source=source.replace('MAX_OBSN* sizeof(float)','MAX_OBSN* sizeof(double)')
 elif name=='choleskyRoutines.C':
  # All three spectral covariance builders initialize imaginary slots 1..N-1
  # but TK_fft also consumes slot zero. A real PSD has zero imaginary DC.
  token='    p_i=(double*)malloc(sizeof(double)*npts);'
  assert source.count(token)==3
  source=source.replace(token,token+'\n    p_i[0]=0; // AE 0053: initialize the omitted imaginary DC sample.')
 elif name=='tempo2Util.C':
  source=source.replace('for (i=0; i<=p[p2].nJumps; i++)','for (i=1; i<=p[p2].nJumps; i++)')
  source=source.replace('for (i=0; i<=p[p2].nfdJumps; i++)','for (i=1; i<=p[p2].nfdJumps; i++)')
  source=source.replace('p2->prefit[i]    = p1.prefit[i];','memcpy(p2->prefit+i,p1.prefit+i,sizeof(longdouble));')
  source=source.replace('p2->prefitErr[i] = p1.prefitErr[i];','memcpy(p2->prefitErr+i,p1.prefitErr+i,sizeof(longdouble));')
 elif name=='t2fit.C':
  source=correct_legacy_global(source)
  start=source.rindex('void subtractTNPoly(');end=source.index('void subtractSWGPMeanSW(',start)
  block=source[start:end]
  token='        paramDerivFunc fitfunc;'
  block=block.replace(token,'''        // No fitted spin term initializes white_y; the constant is unused.
        if (param==param_f && nParams==0) {
            free(psr_x);free(psr_y);free(psr_e);free(white_y);free(psr_toaidx);
            writeResiduals=_writeResiduals;
            return;
        }
''' + token)
  source=source[:start]+block+source[end:]
  source=source.replace('        const unsigned int psr_ndata = t2Fit_getFitData','        if(haveCovar)sortToAs(psr+ipsr);\n        const unsigned int psr_ndata = t2Fit_getFitData')
  source=source.replace('OUT.constraintIndex[OUT.nConstraints]=psr->constraints[ii];','OUT.constraintIndex[OUT.nConstraints]=psr->constraints[i];')
  for field,label in [('TNChrom','chrom'),('TNDM','dm')]:
   start=source.index('for (int i=0;i<psr->'+field+'_log_freqs;++i)')
   stop=source.index('\n            }',start)
   block=source[start:stop];token='=constraint_red_'+label+'_sin;'
   first=block.index(token);second=block.index(token,first+len(token))
   block=block[:second]+block[second:].replace(token,'=constraint_red_'+label+'_cos;',1)
   source=source[:start]+block+source[stop:]
 return source


def correct_legacy_global(source):
 start=source.index('void TKleastSquares_global_pulsar(');end=source.index('void TKfit_getPulsarDesignMatrix(',start)
 block=source[start:end]
 updated=block.replace('constraintsMatrix=malloc_blas(totalConstraints,totalFit);','constraintsMatrix=totalConstraints?malloc_blas(totalConstraints,totalFit):NULL;').replace('    free_blas(constraintsMatrix);','    if(totalConstraints)free_blas(constraintsMatrix);')
 original_call='                CONSTRAINTfuncs(psr,ipsr,nf[ipsr],psr->constraints[ic],constraintsMatrix[ic+off_c]+off_f);'
 assert updated.count(original_call)==1
 updated=updated.replace(original_call,'''                // CONSTRAINTfuncs is ZERO, then psr-zero globals, then locals.
                 // The global design is globals, then each ZERO/local block.
                 const int local_size=nf[ipsr]+(ipsr==0?nglobal:0);
                 double *row=new double[local_size]();
                 CONSTRAINTfuncs(psr,ipsr,local_size,psr[ipsr].constraints[ic],row);
                 if(ipsr==0)for(int g=0;g<nglobal;g++)constraintsMatrix[ic+off_c][g]=row[g+1];
                 constraintsMatrix[ic+off_c][nglobal+off_f]=row[0];
                 for(int j=1;j<nf[ipsr];j++)constraintsMatrix[ic+off_c][nglobal+off_f+j]=row[j+(ipsr==0?nglobal:0)];
                 delete[]row;''')
 return source[:start]+updated+source[end:]
