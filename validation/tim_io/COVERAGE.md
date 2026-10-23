# Measured coverage and withheld branches

Fresh `-O0 --coverage` execution of all 157 declared cases, with gcda removed before each case and gcov JSON collected afterward. `coverage.json` maps every function, executable line and branch to its observed cases. This is separate from the `-O2` numeric reference.

| File | Functions entered | Lines hit/total | Compiler branches hit/total |
|---|---:|---:|---:|
| readJBO_bat.C | 6/6 | 111/111 | 74/103 |
| readTimfile.C | 4/4 | 401/403 | 442/556 |
| read_fortran2.h | 7/7 | 49/50 | 41/52 |
| sortToAs.C | 2/2 | 16/16 | 12/12 |

**Totals: 19/19 routines, 577/580 executable lines, 569/723 compiler edges.** Compiler edge counts include exceptional, library-call and cleanup edges; they are not counts of independent scientific input branches. All 154 unhit compiler edges are listed below, without silently removing them from the denominator.

154 instrumented cases match all ordinary evidence exactly. Three JBO cases (`jbo_records`, `jbo_multi`, `jbo_cvs`) differ in raw word diagnostics at source line 108; full observations are preserved in coverage.json. A separate complete comparison with stdout removed confirms that all their ordered events/state/results, stderr, output bytes, input membership and process status agree. The raw O2 word diagnostics reflect the source type-punning/compiler limit; neither build repairs it.

Unexecuted lines: readTimfile.C:799–800 (negative sqrt-result error path) and read_fortran2.h:49 (actual big-endian host). Compile-time inactive PATH_MAX alternative allocation and commented JBO loop remain source inventory entries, not missing executable lines. Header variants, TTY/dependency failure paths, source-invalid storage and preprocessing are outside this measured primary-source build. SOURCE_MAP.md lists the unsafe/unrun cases.

| Source | Branch index | Source statement | Disposition |
|---|---:|---|---|
| readJBO_bat.C:58 | 3 | `if (displayCVSversion == 1) CVSdisplayVersion("readJBO_bat.C","readJBO_bat()",CVS_verNum);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:60 | 1 | `printf("Attempting to read %s\n",fname);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:61 | 1 | `FILE *batfile = fopen(fname,"r");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:71 | 1 | `fread(&dmyi,1,4,batfile);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:72 | 1 | `fread(&itype,1,4,batfile);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:74 | 3 | `logmsg("Got record type = %d (%d)",itype,++irecord);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:75 | 1 | `fread(raw_record,25,8,batfile);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:76 | 1 | `fread(&dmyi,1,4,batfile);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:80 | 3 | `logmsg("Read BAT file created by %s, ver %s, user %s",cbuf[1],cbuf[2], cbuf[3]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:84 | 3 | `logmsg("Source name: %s%s%s",cbuf[0],cbuf[1],cbuf[2]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:88 | 1 | `swap8(doubles+3);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:89 | 1 | `swap8(doubles+4);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:90 | 1 | `swap8(doubles+5);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:91 | 3 | `logmsg("Observatory name: %s%s%s %lf %lf %lf",cbuf[0],cbuf[1],cbuf[2], doubles[3],doubles[4],doubles[5]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:105 | 3 | `logmsg("%d %d %s",i,i+1,cbuf[i]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:107 | 1 | `swap8(doubles+i);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:108 | 3 | `logmsg("%d %d %lf",i,i+1,doubles[i]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:112 | 3 | `logmsg("site = %s",site);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:114 | 1 | `double diff = (double)(date2mjd((int)doubles[23])*86400.0+doubles[24]) -` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:115 | 1 | `(double)(date2mjd((int)doubles[0])*86400.0+doubles[1]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:117 | 3 | `logmsg("sat-bat = %lg",diff);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:119 | 1 | `longdouble bat = (longdouble)(date2mjd((int)doubles[0])+doubles[1]/86400.0);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:122 | 3 | `logmsg("corr =  = %lf",corr);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:125 | 3 | `logmsg("SAT = %.12lf",(double)psr->obsn[nobs].sat);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:126 | 3 | `logmsg("BAT = %.12lf",(double)psr->obsn[nobs].bat);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:186 | 1 | `fclose(batfile);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:196 | 1 | `imn = (int)fortran_mod(idat / 100,100);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:197 | 1 | `idy = (int)fortran_mod(idat,100);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readJBO_bat.C:199 | 1 | `if (fortran_mod(iyr,4) == 0)` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:64 | 3 | `if (displayCVSversion == 1) CVSdisplayVersion("readTimfile.C","readTimfile()",CVS_verNum);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:66 | 3 | `logdbg("In reading tim file");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:84 | 3 | `logmsg("Reading JBO format!?");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:85 | 1 | `readJBO_bat(timFile[p],&psr[p],p);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:88 | 1 | `readTim(timFile[p],&psr[p],&jumpVal,&fdjumpVal);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:89 | 3 | `logdbg("Checking for deleted points >%s<",psr[0].deleteFileName);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:94 | 3 | `logdbg("In checking for deleted points");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:95 | 1 | `if (!(fin = fopen(psr[0].deleteFileName,"r")))` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:96 | 1 | `printf("Warning: unable to open %s\n",psr[0].deleteFileName);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:103 | 1 | `if (fscanf(fin,"%1023s",inputstr)==1)` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:105 | 1 | `input = parse_longdouble(inputstr);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:116 | 3 | `logdbg("Leaving readTimfile");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:122 | 1 | `char *profileDir = new char[MAX_STRLEN];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:123 | 1 | `char *tt = new char[2*MAX_STRLEN+16];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:125 | 1 | `char *firstWord = new char[1000];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:126 | 1 | `char *line = new char[1000];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:127 | 1 | `char *dummy = new char[1000];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:155 | 1 | `if (!(fin = fopen(timname,"r")))` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:157 | 1 | `printf("ERROR [FILE4]: Unable to open timfile >%s<\n",timname);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:168 | 1 | `fgets(oldLine,1000,fin);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:175 | 1 | `fscanf(fin,"%999s",firstWord);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:181 | 1 | `fclose(fin);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:182 | 1 | `fin = fopen(timname,"r");  /* Could use rewind instead */` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:187 | 1 | `fgets(line,1000,fin); removeCR2(line);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:188 | 1 | `} while (strcasecmp(line,"TOAS")!=0 && feof(fin)==0);` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:203 | 1 | `if (fgets(line,1000,fin)==NULL)             /* Read line from .tim file */` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:208 | 3 | `while (strlen(line) > 0 && isspace(line[strlen(line)-1])) line[strlen(line)-1]='\0';` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:214 | 1 | `char* sat_str = new char[1024];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:262 | 1 | `psr->obsn[nObs].sat_day = parse_longdouble(sat_day_str);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:263 | 1 | `psr->obsn[nObs].sat_sec = parse_longdouble(sat_sec_str);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:265 | 1 | `psr->obsn[nObs].sat = parse_longdouble(sat_str);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:288 | 1 | `for (j=i;j<(int)strlen(line);j++)` | All-space remainder cannot occur here after trailing whitespace trim and successful literal-space value split; a later nonspace ends this loop. |
| readTimfile.C:325 | 1 | `printf("Number of different flags in the .tim file > MAX_FLAGS (%d)\n",MAX_FLAGS);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:331 | 1 | `delete[] sat_str;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:343 | 1 | `else if (format==1) /* Parkes, princeton or ITOA file format */` | Selector produces 0/1/2 and HEAD converts 2 to 1 before this dispatch; remaining format cannot be neither 0 nor 1. |
| readTimfile.C:345 | 1 | `if (fgets(line,1000,fin)==NULL)             /* Read line from .tim file */` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:366 | 1 | `if (strcasecmp(param1,"INCLUDE")!=0)` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:375 | 0 | `strncpy(param1,line+25+add,99); param1[9]='\0'; if (strlen(param1)<2) valid=-2;` | Accepted Parkes length >=79 guarantees the copied 9-byte frequency substring is at least two bytes; no embedded NUL can precede that strlen bound. |
| readTimfile.C:379 | 1 | `psr->obsn[nObs].sat = parse_longdouble(param1);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:396 | 1 | `psr->obsn[nObs].sat = parse_longdouble(param1);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:413 | 1 | `psr->obsn[nObs].sat = parse_longdouble(param1);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:436 | 1 | `&#124;&#124; strcasecmp(psr->obsn[nObs].telID,"bat")==0)` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:501 | 3 | `logdbg("psr %s adding time >%g<",psr->name,time);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:515 | 1 | `printf("Number of different flags in the .tim file > MAX_FLAGS (%d)\n",MAX_FLAGS);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:526 | 1 | `fprintf(stderr, "Too many TOAs! Use the -npsr and -nobs command line arguments!\n");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:535 | 1 | `if (strcasecmp(param1,"END")==0)` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:538 | 1 | `else if (strcasecmp(param1,"PROFILE_DIR")==0)` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:540 | 1 | `else if (strcasecmp(param1,"GLOBAL_EFAC")==0)` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:542 | 1 | `else if (strcasecmp(param1,"EFAC")==0)  /* Error multiplying factor */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:544 | 1 | `else if (strcasecmp(param1,"EFLOOR")==0)  /* Minimum error                        */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:546 | 1 | `else if (strcasecmp(param1,"T2EFAC")==0) /* EFAC for given flag                    */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:553 | 1 | `else if (strcasecmp(param1,"T2EQUAD")==0) /* EQUAD for given flag                  */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:560 | 1 | `else if (strcasecmp(param1,"EMAX")==0)  /* Maximum error                           */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:562 | 1 | `else if (strcasecmp(param1,"EMIN")==0)  /* Minimum error                           */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:564 | 1 | `else if (strcasecmp(param1,"ESET")==0)  /* Set all errors to given amount          */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:566 | 1 | `else if (strcasecmp(param1,"FMAX")==0)  /* Maximum observing frequency             */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:568 | 1 | `else if (strcasecmp(param1,"FMIN")==0)  /* Minimum observing frequency             */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:570 | 1 | `else if (strcasecmp(param1,"INFO")==0)  /* Highlighting flag                       */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:574 | 1 | `else if (strcasecmp(param1,"PHASE")==0) /* Add phase jump                          */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:581 | 1 | `else if (strcasecmp(param1,"EQUAD")==0) /* Error to add in quadrature               */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:583 | 1 | `else if (strcasecmp(param1,"SIGMA")==0) /* Set all errors to constant value         */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:585 | 1 | `else if (strcasecmp(param1,"TIME")==0)  /* Add a constant time to all arrival times */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:590 | 3 | `logdbg("Updating time: %g %g",dtime,time);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:593 | 1 | `else if (strcasecmp(param1,"MODE")==0) /* Fit with errors */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:596 | 1 | `displayMsg(1,"TIM1","Please place MODE flags in the parameter file","",psr->noWarnings);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:598 | 1 | `else if (strcasecmp(param1,"INCLUDE")==0) /* Include another .tim file */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:600 | 1 | `char *newtim = new char[MAX_FILELEN];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:602 | 1 | `char *relPath = new char[PATH_MAX];` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:610 | 1 | `printf("Current filename = %s\n",timname);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:612 | 1 | `realpath(timname,relPath);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:617 | 1 | `for (ii=strlen(relPath);ii>0;ii--)` | Successful realpath is absolute with a parent slash. Loop exhaustion would require the withheld failed/unwritten realpath case. |
| readTimfile.C:626 | 1 | `printf("Rel path = %s\n",relPath);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:628 | 1 | `readTim(relPath,psr,jumpVal,fdjumpVal);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:634 | 1 | `printf("Unable to parse INCLUDE line >%s<\n",line);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:637 | 1 | `delete[] relPath;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:639 | 1 | `delete[] newtim;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:641 | 1 | `else if (strcasecmp(param1,"SKIP")==0) /* Skip data */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:643 | 1 | `else if (strcasecmp(param1,"JUMP")==0) /* JUMP */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:645 | 1 | `else if (strcasecmp(param1,"NOSKIP")==0) /* Stop skipping data */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:650 | 1 | `if (strcasecmp(param1,"NOSKIP")==0) /* Stop skipping data */` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:655 | 1 | `fclose(fin);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:657 | 1 | `delete[] profileDir;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:658 | 1 | `delete[] tt;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:659 | 1 | `delete[] firstWord;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:660 | 1 | `delete[] line;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:661 | 1 | `delete[] dummy;` | Compiler cleanup/null edge on delete[] of successful throwing-new allocation. Allocation failure/null substitution not injected. |
| readTimfile.C:676 | 1 | `if (!(fout = fopen(timname,"w")))` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:678 | 1 | `printf("Sorry, unable to write to file %s\n",timname);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:681 | 3 | `if (strcmp(fileFormat,"tempo2")==0) fprintf(fout,"FORMAT 1\n");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:682 | 3 | `if (psr->fitMode==1) fprintf(fout,"MODE 1\n");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:686 | 1 | `fprintf(fout,"GLOBAL_EFAC %.5g\n",psr[0].T2globalEfac);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:689 | 1 | `fprintf(fout,"T2EQUAD %s %s %.5g\n",psr[0].T2equadFlagID[i],psr[0].T2equadFlagVal[i],psr[0].T2equadVal[i]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:692 | 1 | `fprintf(fout,"T2EFAC %s %s %.5g\n",psr[0].T2efacFlagID[i],psr[0].T2efacFlagVal[i],psr[0].T2efacVal[i]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:697 | 1 | `fprintf(fout,"EQUAD %lf\n",current_equad);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:699 | 1 | `fprintf(fout,"EFAC %lf\n",current_efac);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:704 | 1 | `fprintf(fout,"EQUAD %lf\n",psr[0].obsn[i].equad);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:708 | 1 | `fprintf(fout,"EFAC %lf\n",psr[0].obsn[i].efac);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:712 | 1 | `fprintf(fout,"C");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:729 | 1 | `lookup_observatory_alias(psr->tOffsetSite[k], offsetSite);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:730 | 1 | `lookup_observatory_alias(psr->obsn[i].telID, obsSite);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:771 | 1 | `v = parse_longdouble(psr->obsn[i].flagVal[k]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:782 | 1 | `v = parse_longdouble(psr->obsn[i].flagVal[k]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:797 | 1 | `oldsat, interim_error,psr->obsn[i].telID);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:798 | 0 | `if(interim_error<longdouble(0.0)){` | Ancestor requires sqrt result < 0. Ordinary IEEE sqrt gives nonnegative values or NaN; that source exit is unreachable in this arithmetic profile. |
| readTimfile.C:799 | 0 | `logerr("TOAerror < 0!!\n");` | Ancestor requires sqrt result < 0. Ordinary IEEE sqrt gives nonnegative values or NaN; that source exit is unreachable in this arithmetic profile. |
| readTimfile.C:799 | 1 | `logerr("TOAerror < 0!!\n");` | Ancestor requires sqrt result < 0. Ordinary IEEE sqrt gives nonnegative values or NaN; that source exit is unreachable in this arithmetic profile. |
| readTimfile.C:799 | 2 | `logerr("TOAerror < 0!!\n");` | Ancestor requires sqrt result < 0. Ordinary IEEE sqrt gives nonnegative values or NaN; that source exit is unreachable in this arithmetic profile. |
| readTimfile.C:799 | 3 | `logerr("TOAerror < 0!!\n");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:806 | 1 | `if (strcasecmp(psr->obsn[i].flagID[j],"-selectAll")!=0)` | Additional gcov call-side edge on libc strcasecmp. The source match and nonmatch outcomes are both exercised; this call-side edge was not forced or claimed reachable. |
| readTimfile.C:807 | 1 | `fprintf(fout,"%s %s ",psr->obsn[i].flagID[j],psr->obsn[i].flagVal[j]);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:809 | 1 | `fprintf(fout,"\n");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:818 | 1 | `psr[0].obsn[i].sat,(double)psr[0].obsn[i].origErr,psr[0].obsn[i].telID);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:819 | 1 | `fprintf(fout,"\n");` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| readTimfile.C:830 | 1 | `fclose(fout);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:46 | 1 | `if (intcnv.asChar[0]==1)` | Actual-host big-endian detection branch; this host is little-endian. Controlled swap modes exercise readers, not alternate-host execution. |
| read_fortran2.h:52 | 1 | `printf("Byte swapping = %d\n",swapByte2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:62 | 1 | `if (!(c_fileptr2 = fopen(fname,"rb")))` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:64 | 1 | `printf("C PROGRAM: Unable to open filename >>%s<<\n",fname);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:101 | 1 | `intcnv.asChar[i]=fgetc(c_fileptr2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:107 | 1 | `intcnv.asChar[i]=fgetc(c_fileptr2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:126 | 1 | `intcnv.asChar[i]=fgetc(c_fileptr2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:132 | 1 | `intcnv.asChar[i]=fgetc(c_fileptr2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:153 | 1 | `dblcnv.asChar[i]=fgetc(c_fileptr2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:161 | 1 | `dblcnv.asChar[i]=fgetc(c_fileptr2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
| read_fortran2.h:172 | 1 | `fread(&ival,sizeof(int),1,c_fileptr2);` | Compiler-marked exception edge; allocation/I/O/C++ unwinding faults not injected. |
