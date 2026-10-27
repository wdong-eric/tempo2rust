// Instrumentation only: one slot per source observation, no shared increment.
#include "tempo2.h"
#include <omp.h>
#include <vector>
#include <set>
#include <string>
#include <cstdio>
#include <cstdlib>
static std::vector<std::vector<int>> ids, visits;
static std::vector<std::vector<std::vector<std::string>>> events;
static thread_local int current_p=-1, current_i=-1;
void ak_begin(pulsar* psr, int n) {
    ids.clear(); visits.clear(); events.clear();
    for(int p=0;p<n;p++) { ids.emplace_back(psr[p].nobs,-1); visits.emplace_back(psr[p].nobs,0); events.emplace_back(psr[p].nobs); }
}
void ak_visit(int p,int i) { current_p=p; current_i=i; ids[p][i]=omp_get_thread_num(); visits[p][i]++; }
void ak_end() {
    for(size_t p=0;p<ids.size();p++) {
        std::set<int> workers;
        for(size_t i=0;i<ids[p].size();i++) {
            if(visits[p][i]!=1 || ids[p][i]<0) std::abort();
            workers.insert(ids[p][i]);
            for(size_t e=0;e<events[p][i].size();e++)
                std::fprintf(stderr,"AK_EVENT pulsar=%zu observation=%zu event=%zu %s\n",p,i,e,events[p][i][e].c_str());
        }
        std::fprintf(stderr,"AK_WORKERS pulsar=%zu actual=%zu observations=%zu\n",p,workers.size(),ids[p].size());
    }
}

void ak_sheet(const char* path) { events[current_p][current_i].push_back(std::string("resource ")+path); }
void ak_solar(int r1,int r2,double b,double dm,int slow,int fast,double elong) {
    char text[512];
    std::snprintf(text,sizeof(text),"solar %d %d %a %a %d %d %a",r1,r2,b,dm,slow,fast,elong);
    events[current_p][current_i].push_back(text);
}
