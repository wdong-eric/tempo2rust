"""Add complete initialized per-iteration observation snapshots to the observer only."""
import subprocess
from inventory import HERE,sha,save
import profile as native
from profile import compile_probe,ENV

def build():
 # Ordinary source and its frozen builder stay immutable. Resolve the two
 # declaration anchors to the raw Git keyword, not the checkout's smudged Id.
 patches=[]
 for file,old,new in native.OBSERVERS:
  if old.startswith('const char *CVS_verNum'):
   raw='const char *CVS_verNum = "$Id$";'
   new=new.replace(old,raw);old=raw
  patches.append((file,old,new))
 native.OBSERVERS=patches
 exe,p=compile_probe('observer',observer=True)
 caller=exe.parent/'observer_probe.cc'
 source=(HERE/'probe.cc').read_text()
 old='void ab_iteration(int p,int i,int loop,double old,double now,double delt){'
 new='''static std::string iterationObservation(const observation&x){auto retained=std::move(fields);fields.clear();dump("",x);std::ostringstream o;o<<'{';bool comma=false;for(auto&kv:fields){if(comma)o<<',';comma=true;o<<quote(kv.first.substr(1))<<':'<<kv.second;}o<<'}';fields=std::move(retained);return o.str();}
void ab_iteration(int p,int i,int loop,double old,double now,double delt){'''
 assert source.count(old)==1
 source=source.replace(old,new)
 old_end='+scalar(delt)+"}");}'
 new_end='+scalar(delt)+",\\\"observation\\\":"+iterationObservation(ps[p].obsn[i])+"}");}'
 assert source.count(old_end)==1
 source=source.replace(old_end,new_end)
 # Direct mcl2 outputs are explicitly initialized by the caller. Register only
 # their live scope; serialize immediately on return, or at exit before unwind.
 aux=r'''static double *aux_h=nullptr,*aux_c=nullptr,*aux_r=nullptr,*aux_v=nullptr;
static int* aux_count=nullptr;
static std::string aux_saved="null";
static std::string diagnosticRecord(){auto retained=std::move(fields);fields.clear();dump("errors",TK_errorlog);dump("warnings",TK_warnlog);std::ostringstream o;o<<'{';bool comma=false;for(auto&kv:fields){if(comma)o<<',';comma=true;o<<quote(kv.first)<<':'<<kv.second;}o<<'}';fields=std::move(retained);return o.str();}
static std::string auxRecord(){auto retained=std::move(fields);fields.clear();cells("helat",aux_h,36);cells("crlon",aux_c,36);cells("rotations",aux_r,36);cells("scalars",aux_v,7);dump("count",*aux_count);std::ostringstream o;o<<'{';bool comma=false;for(auto&kv:fields){if(comma)o<<',';comma=true;o<<quote(kv.first)<<':'<<kv.second;}o<<'}';fields=std::move(retained);return o.str();}
'''
 anchor='static void privateState(std::ostream&o)'
 assert source.count(anchor)==1;source=source.replace(anchor,aux+anchor)
 finish='o<<"]}";extra.clear();}'
 amended=r'''o<<"],\"mcl_outputs\":"<<(aux_h?auxRecord():aux_saved)<<",\"diagnostic_buffers\":"<<diagnosticRecord()<<'}';extra.clear();aux_saved="null";}'''
 assert source.count(finish)==1;source=source.replace(finish,amended)
 register='for(double&x:v)x=84;mcl2('
 registered='for(double&x:v)x=84;aux_h=h;aux_c=c;aux_r=t;aux_v=v;aux_count=&count;mcl2('
 assert source.count(register)==1;source=source.replace(register,registered)
 returned='v+6,&count);r<<count;'
 captured='v+6,&count);aux_saved=auxRecord();aux_h=nullptr;aux_c=nullptr;aux_r=nullptr;aux_v=nullptr;aux_count=nullptr;r<<count;'
 assert source.count(returned)==1;source=source.replace(returned,captured)
 caller.write_text(source)
 # Recompile only the caller and relink the exact freshly built observer TUs.
 # The preliminary caller object/executable is replaced before any launch.
 command=p['commands'][0].copy();command[command.index('-c')+1]=str(caller)
 for cmd in [command,p['commands'][-1]]:
  r=subprocess.run(cmd,capture_output=True,text=True,env=ENV);assert r.returncode==0,r.stderr
 p['commands'][0]=command
 p['observer_caller_patch']=[[old,new],[old_end,new_end],[anchor,aux+anchor],[finish,amended],[register,registered],[returned,captured]]
 p['dependencies'][str(caller)]=sha(caller.read_bytes())
 p['dependencies'][str(HERE/'observer_profile.py')]=sha((HERE/'observer_profile.py').read_bytes())
 p['executable_sha256']=sha(exe.read_bytes());p['link_map_sha256']=sha((exe.parent/'probe.map').read_bytes())
 save(exe.parent/'profile.json',p)
 return exe,p
