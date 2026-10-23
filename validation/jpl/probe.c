/* Assignment H observation harness. Counterparts: canonical jpleph.c (included
 * unchanged), jpl_int.h layouts, jpleph.h API. No production implementation.
 * Commands are trusted generated fixtures; caller buffers deliberately exceed
 * the source's documented nut[4] because librations write six components. */
#define _POSIX_C_SOURCE 200809L
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <stddef.h>
#include <string.h>
#include <math.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>

static int fail_seek, fail_read, fail_alloc;
static unsigned seeks, reads, allocs;
static size_t last_read_size, last_read_requested, last_read_result;
static int observed_seek(FILE *f, long off, int whence) {
    seeks++;
    if (fail_seek && !--fail_seek) return -1;
    return fseek(f, off, whence);
}
static size_t observed_read(void *p, size_t size, size_t n, FILE *f) {
    reads++;
    last_read_size=size; last_read_requested=n;
    if (fail_read && !--fail_read) last_read_result=0;
    else last_read_result=fread(p, size, n, f);
    return last_read_result;
}
static void *observed_alloc(size_t n, size_t size) {
    allocs++;
    if (fail_alloc && !--fail_alloc) return NULL;
    return calloc(n, size);
}
/* Faults are off unless an input command explicitly enables one. These are
 * observational libc boundaries, not edits to the canonical source. */
#define fseek observed_seek
#define fread observed_read
#define calloc observed_alloc
#include "jpleph.c"
#undef fseek
#undef fread
#undef calloc

static void number(double x) {
    uint64_t u;
    memcpy(&u, &x, sizeof u);
    const char *kind = isnan(x) ? "NaN" : isinf(x) ? (signbit(x) ? "-Infinity" : "+Infinity") : "finite";
    printf("{\"class\":\"%s\",\"decimal\":\"", kind);
    if (isfinite(x)) printf("%.17g", x); else printf("%s", kind);
    printf("\"");
    if (isfinite(x)) printf(",\"bits\":\"%016llx\"", (unsigned long long)u);
    printf("}");
}
static void numbers(const double *p, size_t n) {
    putchar('[');
    for (size_t i = 0; i < n; i++) { if (i) putchar(','); number(p[i]); }
    putchar(']');
}
static void bytes(const void *p, size_t n) {
    const unsigned char *b = p;
    putchar('"');
    for (size_t i = 0; i < n; i++) printf("%02x", b[i]);
    putchar('"');
}
static void fill(double *p, size_t n) { for (size_t i = 0; i < n; i++) p[i] = -9000.0 - (double)i; }

static void snapshot(struct jpl_eph_data *e, int controlled) {
    printf(",\"cache_loc\":%u,\"cache_bytes\":", e->curr_cache_loc);
    bytes(e->cache, e->recsize);
    if (controlled) {
        printf(",\"sun_epoch\":"); number(e->pvsun_t);
        printf(",\"sun\":"); numbers(e->pvsun, 9);
        printf(",\"n_posn\":%u,\"n_vel\":%u,\"twot\":", e->iinfo.n_posn_avail, e->iinfo.n_vel_avail);
        number(e->iinfo.twot);
        printf(",\"posn_coeff\":"); numbers(e->iinfo.posn_coeff, MAX_CHEBY);
        printf(",\"vel_coeff\":"); numbers(e->iinfo.vel_coeff, MAX_CHEBY);
    }
}
static void header(struct jpl_eph_data *e) {
    double d[] = {e->ephem_start, e->ephem_end, e->ephem_step, e->au, e->emrat};
    printf(",\"header_doubles\":"); numbers(d, 5);
    printf(",\"ncon\":%u,\"version\":%u,\"kernel_size\":%u,\"recsize\":%u,\"ncoeff\":%u,\"swap\":%u,\"ipt\":[",
           e->ncon,e->ephemeris_version,e->kernel_size,e->recsize,e->ncoeff,e->swap_bytes);
    for (int i=0;i<15;i++) printf("%s[%u,%u,%u]",i?",":"",e->ipt[i][0],e->ipt[i][1],e->ipt[i][2]);
    printf("]");
}

int main(void) {
    struct jpl_eph_data *e = NULL;
    int controlled = 0;
    unsigned event = 0;
    char line[8192], op[32], path[4096];
    printf("{\"op\":\"startup\",\"init_error\":%d,\"struct_size\":%zu,\"double_align\":%zu,\"pvsun_offset\":%zu,\"macro_offset\":248,\"au_offset\":%zu,\"iinfo_offset\":%zu,\"ifile_offset\":%zu}\n",
           jpl_init_error_code(),sizeof(struct jpl_eph_data),_Alignof(double),offsetof(struct jpl_eph_data,pvsun),offsetof(struct jpl_eph_data,au),offsetof(struct jpl_eph_data,iinfo),offsetof(struct jpl_eph_data,ifile));
    while (fgets(line,sizeof line,stdin)) {
        if (sscanf(line,"%31s",op)!=1) continue;
        printf("{\"event\":%u,\"op\":\"%s\"",event++,op);
        if (!strcmp(op,"open")) {
            int flags;
            char names[2048][6]; double values[2048];
            memset(names,'?',sizeof names); fill(values,2048);
            sscanf(line,"%*s %4095s %d",path,&flags);
            e=jpl_init_ephemeris(path,flags&1?names:NULL,flags&2?values:NULL);
            controlled=0;
            printf(",\"nonnull\":%s,\"init_error\":%d",e?"true":"false",jpl_init_error_code());
            if(e) {
                header(e);
                if(flags&1) { printf(",\"name_bytes\":"); bytes(names,e->ncon*6); }
                if(flags&2) { printf(",\"constants\":"); numbers(values,e->ncon); }
            }
        } else if (!strcmp(op,"control")) {
            /* Explicit experimental state, never an initializer repair. */
            e->pvsun_t=NAN;
            fill(e->pvsun,9);
            e->iinfo.twot=0;
            e->iinfo.n_posn_avail=e->iinfo.n_vel_avail=2;
            for(int i=2;i<MAX_CHEBY;i++) e->iinfo.posn_coeff[i]=e->iinfo.vel_coeff[i]=0;
            controlled=1;
            snapshot(e,controlled);
        } else if (!strcmp(op,"state")) {
            double et, pv[16][6], nut[6]; int bary,list[14];
            sscanf(line,"%*s %lf %d %d %d %d %d %d %d %d %d %d %d %d %d %d %d",&et,&bary,
                list,list+1,list+2,list+3,list+4,list+5,list+6,list+7,list+8,list+9,list+10,list+11,list+12,list+13);
            fill(&pv[0][0],96); fill(nut,6);
            int s=jpl_state(e,et,list,pv,nut,bary);
            printf(",\"status\":%d,\"pv\":",s); numbers(&pv[0][0],96);
            printf(",\"nut\":"); numbers(nut,6); snapshot(e,controlled);
        } else if (!strcmp(op,"state_nut4")) {
            double pv[16][6],nut[4]; int list[14]={0};
            fill(&pv[0][0],96); fill(nut,4); list[11]=2;
            int s=jpl_state(e,e->ephem_start+1,list,pv,nut,1);
            printf(",\"status\":%d",s);
        } else if (!strcmp(op,"pleph") || !strcmp(op,"pleph_status")) {
            double et,out[6]; int target,centre,velocity;
            sscanf(line,"%*s %lf %d %d %d",&et,&target,&centre,&velocity); fill(out,6);
            int s=jpl_pleph(e,et,target,centre,out,velocity);
            printf(",\"status\":%d",s);
            if(!strcmp(op,"pleph") && !s) { printf(",\"out\":"); numbers(out,6); }
            /* Failed pleph local pv is uninitialized: do not read its output. */
            if(controlled) snapshot(e,controlled);
        } else if (!strcmp(op,"constant")) {
            int idx; char name[7]; memset(name,'?',sizeof name);
            sscanf(line,"%*s %d",&idx);
            double v=jpl_get_constant(idx,e,name);
            printf(",\"name_bytes\":"); bytes(name,7); printf(",\"value\":"); number(v);
        } else if (!strcmp(op,"getter")) {
            int offset; char type; sscanf(line,"%*s %c %d",&type,&offset);
            printf(",\"value\":");
            if(type=='d') number(jpl_get_double(e,offset)); else printf("%ld",jpl_get_long(e,offset));
        } else if (!strcmp(op,"fault")) {
            char kind[20]; int n; sscanf(line,"%*s %19s %d",kind,&n);
            if(!strcmp(kind,"seek")) fail_seek=n;
            if(!strcmp(kind,"read")) fail_read=n;
            if(!strcmp(kind,"alloc")) fail_alloc=n;
        } else if (!strcmp(op,"truncate")) {
            long n; sscanf(line,"%*s %4095s %ld",path,&n);
            if(truncate(path,n)) return 20;
        } else if (!strcmp(op,"replace")) {
            char other[4096]; sscanf(line,"%*s %4095s %4095s",path,other);
            if(rename(other,path)) return 21;
        } else if (!strcmp(op,"overwrite")) {
            char other[4096]; sscanf(line,"%*s %4095s %4095s",path,other);
            FILE *src=fopen(other,"rb"); int fd=open(path,O_WRONLY); unsigned char b[8192];
            size_t n; off_t off=0;
            if(!src || fd<0) return 22;
            while((n=fread(b,1,sizeof b,src))) { if(pwrite(fd,b,n,off)!=(ssize_t)n) return 23; off+=n; }
            if(ftruncate(fd,off)) return 24;
            fclose(src); close(fd);
        } else if (!strcmp(op,"close")) {
            jpl_close_ephemeris(e); e=NULL; controlled=0;
            printf(",\"init_error\":%d",jpl_init_error_code());
        } else if (!strcmp(op,"interp")) {
            double fraction,step,coef[432],out[9]; unsigned ncf,ncm,na; int velocity;
            sscanf(line,"%*s %lf %lf %u %u %u %d",&fraction,&step,&ncf,&ncm,&na,&velocity);
            struct interpolation_info info={0};
            info.posn_coeff[0]=1; info.posn_coeff[1]=-2; info.vel_coeff[1]=1;
            for(unsigned i=0;i<432;i++) coef[i]=(i%7==0?-1.0:1.0)*(i+1)/16.0;
            fill(out,9); double t[]={fraction,step};
            interp(&info,coef,t,ncf,ncm,na,velocity,out);
            printf(",\"out\":"); numbers(out,9);
        } else if (!strcmp(op,"helpers")) {
            unsigned char b[]={0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15};
            swap_32_bit_val(b); swap_64_bit_val(b,2);
            printf(",\"swap_bytes\":"); bytes(b,16);
            printf(",\"dimensions\":["); for(int i=0;i<16;i++) printf("%s%d",i?",":"",dimension(i)); printf("]");
        } else return 26;
        if(e && (!strcmp(op,"open") || !strcmp(op,"truncate") || !strcmp(op,"replace") || !strcmp(op,"overwrite"))) {
            struct stat handle_info,path_info;
            if(fstat(fileno(e->ifile),&handle_info) || stat(path,&path_info)) return 27;
            printf(",\"path_matches_open_inode\":%s,\"open_file_size\":%lld,\"path_file_size\":%lld",
                handle_info.st_dev==path_info.st_dev && handle_info.st_ino==path_info.st_ino?"true":"false",
                (long long)handle_info.st_size,(long long)path_info.st_size);
        }
        printf(",\"io_calls\":[%u,%u,%u],\"last_fread_size_requested_returned\":[%zu,%zu,%zu]}\n",
            seeks,reads,allocs,last_read_size,last_read_requested,last_read_result);
        fflush(stdout);
    }
    if(e) jpl_close_ephemeris(e);
    return 0;
}
