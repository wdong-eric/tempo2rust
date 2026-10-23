"""Declared bounded caller histories, frozen before native experiments."""
def cases():
    out=[]
    def add(name,args,classification='defined',status=0):out.append(dict(id=name,args=[str(x) for x in args],classification=classification,returncode=status))
    for full in [-7,0,1,9]:add('natural_'+str(full),['natural',full,2])
    for count in [0,1,2]:add('wrapper_'+str(count),['wrapper',count,-3])
    add('wrapper_debug_cvs',['wrapper',1,1,1,1],'host_abi')
    add('seeded_reinitialise',['repeat',-4,7])
    for arg in [-3,0,1,5]:add('allocate_'+str(arg),['allocate',arg])
    add('destroy_constraints',['destroy'])
    add('destroy_null_observation',['destroy_null'])
    for rows,cols in [(1,1),(2,3),(3,2),(4,4)]:add('matrix_%d_%d'%(rows,cols),['matrix',rows,cols])
    add('matrix_uinv',['uinv',3])
    add('matrix_zero_rows',['matrix',0,3],'assert_termination',-6)
    add('matrix_zero_cols',['matrix',3,0],'assert_termination',-6)
    # Source counts nobs/nJumps and destination nfdJumps; source FD count differs.
    for name,n,j,f,sf in [('negative',-1,-1,-1,2),('zero',0,0,0,2),('partial',2,2,3,1),('last',3,1999,1999,0)]:
        add('copy_'+name,['copy',n,j,f,sf,'disjoint'])
    for alias in ['shared','forward_overlap','backward_overlap']:
        add('copy_'+alias,['copy',3,1,2,0,alias])
    add('copy_repeat',['copy_repeat',2,1,2,0,'disjoint'])
    return out
if __name__=='__main__':
    from inventory import HERE,save
    save(HERE/'cases.json',cases())
