# Assignment V frozen source, format and caller contract

Frozen before native experiments, from central release 8e56df0 and raw TEMPO2
fda6ad4a9753ad7dc5d291406f2f310a9161423f. Decisions 0008, 0012, 0039 and 0047
apply. Accepted J/M/N/S references are read-only preparation context. T/U and
central working files are excluded from immutable evidence membership.

## Complete routine and branch map

Line numbers are in raw source. Loops list their continuation/termination edges;
short-circuit expressions retain each predicate. The generated local inventory
will bind these ranges to raw hashes and every compiler edge separately.

| Source/routine | Lines | Branches and callers |
|---|---:|---|
| ifteph.C IFTswap4 | 69–75 | no condition; Int/Ints call; caller 4 initialized bytes |
| IFTswapInt | 77–81 | calls swap4; init ncon |
| IFTswapInts | 83–89 | i<n (87); init six descriptors; n<=0 no-op |
| IFTswap8 | 91–99 | no condition; swapDouble |
| IFTswapDouble | 101–105 | calls swap8; init three dates/two constants |
| IFTswap8N | 107–120 | i<n (112); swapDoubles; n<=0 no-op |
| IFTswapDoubles | 122–126 | calls swap8N; get_Vals record |
| IFTE_init | 128–195 | !f 141; swap 159; ncon!=2 160; swap 166/174/182/185; direct/init_ifte |
| IFTE_close_file | 197–201 | f!=NULL 199; direct/tt2tb |
| IFTE_get_Vals | 204–284 | date<start 236; date==end 246; irec!=cache 257; fseek<0 259; short read 265; swap 273; kind==1 277; all convenience routines |
| IFTE_get_DeltaT_DeltaTDot | 287–294 | no conditions; scalar wrappers |
| IFTE_DeltaT | 296–301 | no conditions; IF_deltaT |
| IFTE_DeltaTDot | 303–308 | no conditions; tt2tb dilation |
| IFTE_get_vE_vEDot | 310–323 | i<3 318; interleaved position/derivative writes; wrappers |
| IFTE_get_vE | 325–329 | no conditions; direct |
| IFTE_get_vEDot | 331–335 | no conditions; tt2tb dilation |
| IFTEinterp | 388–476 | tc changed 417; np<ncf 428; polynomial loop 432; components 437; reverse sum 442; ifl<=1 446; nv<ncf 453; derivative loop 458; components 465; reverse sum 471 |
| tt2tdb.C tt2tb | 65–194 | CVS 74; p<npsr 77; i<nobs 79; clockCorr==0 && telID!=STL 81; STL 85; IF99 90; FB90 94; useCalceph==1 99; k<3 113; SI 120; TDB 130; dilation==1 142; first==1 144; SI 162; TDB 167; literal 1 at 133 has inactive false arm |
| init_ifte | 197–208 | no active conditions; unconditional root concatenate/init |
| IF_deltaT | 210–215 | no conditions; integer split, source factor 86400 |
| FB_deltaT | 217–308 | half-day >=.5 250; nr<1 || tdt>end 261; nr!=tdbnrl 266; j<nr-1 270; k<ncf 272; tc!=pc[1] 290; np<ncf 296; k<=ncf 298; reverse j>=1 303 |
| read_fortran.h open_file | 35–65 | host union byte 46; fname[i] 51; blank 53; !fopen 59; FB/direct |
| close_file | 66–69 | unconditional fclose; FB/direct |
| read_character | 71–76 | i<len 74; <=0 no-op |
| read_char | 77–80 | direct fgetc narrowed to char |
| read_int | 82–103 | swap==1 92; reverse loop 94; forward loop 99; FB/direct |
| read_float | 105–125 | swap==1 114; reverse 116; forward 121; direct |
| read_double | 127–148 | swap==1 137; reverse 139; forward 144; FB/direct |
| read_record_int | 149–154 | unchecked native-order fread; direct complete read only |

All 29 are active. HAVE_CONFIG_H selects the pinned three-line configuration.
Header guards and __cplusplus linkage do not add runtime branches. Historical
commented resets, vector code, old K value and debug printf are inactive.
Clock callers in global.C clock_corrections and ephemeris_routines call tt2tb
before/after planetary calculations; several plugins call it directly. They are
source context, not executed timing pipelines. Other users of IFTE constants
include units.C/readEphemeris.C/readEphemeris_calceph.C; those are not transferred.

## Binary layout and arithmetic contracts

IFTE fields, byte offsets: TTL[252] at 0, CNAM[12] at 252, three binary64 dates
at 264/272/280, int32 ncon at 288, six int32 ipt at 292–315. No TTL bytes are
copied into ifte.title. Both byte orders use ncon==2 as discriminator. Each
record has R=8*(ipt[1][0]-1+3*ipt[1][1]*ipt[1][2]) bytes. Constants at R:
binary64 ephver (floor then int), binary64 L_C. Coefficients start at 2R;
ipt offsets are one-based doubles. Each group is [subinterval][component][ncf].
Require R>=316, R/8<=322, positive finite step, ncf 0..18, positive na,
valid coefficient extents and bounded seek products. Main synthetic layout is
ncf=4, na=4, ipt=(3,4,4,19,4,4), R=528. Alternate maximum layout uses ncf=18,
na=1, ipt=(3,18,1,21,18,1), R=592. Spare records permit dates above end without
adding an upper-date guard. Exact end with positive fraction is tested only
where subinterval pointers stay within initialized backing.

Raw real IF99 is big-endian, dates 2438736.5..2469808.5 step 32,
ipt=(3,7,8,59,7,8), R=1808; 1,759,184 bytes. The actual file is independently
decoded and hashed, then copied to a local runtime for bounded smoke cases.

FB is not a generic record-marker reader. Its exact header is binary64 start,
end; int32 width,count; five ignored binary64, total 64 bytes. After this,
the code reads (nr-1)*count doubles sequentially, retaining only the last
count. No record-marker interpretation occurs. Source open_file selects big
endian on this host. Raw real FB header is (2433282,2469794,16,8), 146,112 bytes.
Fixtures supply full 64-byte headers and enough complete coefficient blocks.
Positive width, count -1/0..16, bounded finite dates/int casts/loop arithmetic.
nr==1 reads no block, retaining static buf zeros or previous coefficients.
For count<=0 no buf/pc coefficient reads occur, returning +0. Changing count
upwards after a same-record reuse exposes initialized static tail retention.

IFTE normalization floors both split epoch parts, carries their fractions and
checks only normalized JDeph0. l=int(na*t0-trunc(t0)); tc=2*(frac(na*t0)+trunc(t0))-1.
Both conversions and coefficient offsets must be valid before execution.
Reverse summation and derivative scale 2*na/t1 remain unchanged. At t0==1,
l=na-1 and tc=1. Reinit resets pc[1] to zero but retains twot: the next tc=0
can use stale twot. This is an intentional witness, not a polynomial identity.

Header constants: JD0=2443144.5003725, MJD0=43144.0003725,
TEPH0=-65.564518e-6, LC=1.48082686742e-8, KM1=1.55051979176e-8,
K=(longdouble)1+(longdouble)KM1. Decimal macros first have their C double
meaning; extended construction is not a fresh high-precision decimal parse.
SI=1, TDB=2, IF99=1, FB90=2. SECDAY and resource suffixes come from tempo2.h.

## State, lifetime and observation projection

Static ifte begins zero initialized. Export all fields: title[256], dates,
ephver, L_C, swap_endian, reclen, irec, buf[322], pc[18], vc[18], twot,np,nv,
ipt[2][3]. Export pointers only as caller-maintained owner state. Init writes
header/constants/new owner and np=2,nv=3,pc[0]=1,pc[1]=0,vc[1]=1,irec=-1.
All other cells retain known static/history values. Replacement does not close
the old handle: caller saves it, exports live ftell/eof/error and closes it in
a separate cleanup action. close leaves a released non-null native pointer;
never inspect or dereference it. Initially null close is safe. Successful init
after close creates a fresh owner. irec remains -1 after every native read.
Snapshots never consume bytes or call value getters.

Fortran c_fileptr/swapByte start zero; open changes swapByte then mutates the
first space in caller filename to NUL without clearing its suffix. Failed
open leaves null, returns 1. Complete mutable filename/output buffers and
tails are exported. close leaves a released stream. FB's private function
statics tdbnrl=-1 and buf[16]=0 cannot be directly accessed without source
instrumentation: no accessor is fabricated. Their complete knownness and
observable consequences are recorded through ordered calls, inputs and exact
results. Local pc and unused locals are not observed. A separate mutation
and coverage build do not replace ordinary observations.

Caller allocates two value-initialized pulsars with four initialized observations
each. Export every explicitly initialized/used field, including all vector,
clock-chain, telID and output backing/tails, plus scalar controls and retained
observations beyond nobs. Remaining aggregate-initialized fields are invariant
zero/null by C++ initialization and never read by selected functions; no padding
is serialized. Source read/write inventory: nobs,timeEphemeris,useCalceph,units,
dilateFreq; obs sat,clockCorr,telID,nclock_correction,correctionsTT[].correction,
earth_ssb[3:6],observatory_earth[3],siteVel[3],correctionTT_calcEph,
correctionTT_TB,correctionTT_Teph,einsteinRate. Pulsar zero always has four live
slots for the cross-pulsar CALCEPH read. These are supplied values, no CALCEPH
library execution. Bypass writes TB only. deltaT is once-per-call local and
can retain across pulsars with unmatched timeEphemeris. first is per call;
IFTE initialization/close occurs even with zero/negative counts. Dilation
always uses IFTE derivatives. The second scaling targets obsTerm, whose later
value is unused: ordinary output cannot expose removal of that dead assignment;
changing its target to obsTermDot does have an output witness. Report this limit.

Actual dependencies: full raw TKlongdouble.C, tempo2Util.C, TKlog.C, clkcorr.C,
tabulatedfunction.C, observatory.C plus their included headers; verbatim
global.C data prefix before extra_delays and CVSdisplayVersion through EOF.
No initializer or reference archive linked. Header selection uses verbatim
TKlongdouble.float128.h as TKlongdouble.h. The A2 config only supplies build
macros; verify its hash. Dependencies compiled but not entered remain outside
coverage claims. Diagnostics inherit M's host-qualified logger constraints.

## Native-invalid cases classified before execution

UNRUN: partial IFTE header/descriptor/constants reads (unchecked/uninitialized
locals); malformed widths/counts causing zero divisor then invalid cast,
negative/overflowing record length/seek arithmetic, buffer>322, ncf>18 or
negative ncf reverse loops, invalid kind descriptor pointers, bad cache np/nv,
invalid subinterval/coefficient/output extents. UNRUN: NaN/infinity/out-of-int
range dates/headers; pointer/size arithmetic overflow. UNRUN: null/released
stream reads, double close, direct close_file before open, getter before init,
FB missing-open wrapper, unset environment, overlong paths, unterminated strings,
uninitialized caller fields, excessive npsr/nobs/clock counts or CALCEPH backing.
UNRUN: read_record_int short/EOF (uninitialized local). UNRUN: FB count>16,
width zero, malformed/truncated headers, arithmetic overflow, invalid casts.
No signal/sanitizer/result is claimed for these.

Executable source exits: complete IFTE header with invalid ncon, failed IFTE
open, query before start, complete header with short coefficient record, and
FB nr<1/date>end. fseek failure is defined with an appropriate live stream,
but ordinary regular files do not provoke it; classify still-defined/unverified
unless a bounded valid stream fixture is separately justified. IFTE cache-hit
skip is unreachable under normal successful init/get histories because irec
never changes; do not seed source state to pretend it is reachable.

Host-qualified: signed-char fgetc EOF narrowing, union-based numeric reads,
byte-swapped scalar storage, native record-int order, stdio buffer/file-edit
visibility, logs and raw diagnostics. EOF byte readers assign every byte;
all-ones integer and quiet-NaN float/double representations are only executed
after host widths/representation measurements. No partial int fread.
Other hosts/locales, concurrency, fault injection/resource exhaustion and full
planetary/BAT/residual pipelines are outside this release.

## Dependency-ordered implementation and completion

1. This contract and inventory generator freeze source/branch/state/format/safety.
2. profile.py/probe.cc build raw sources and export exact ordered initialized state.
3. fixtures.py defines synthetic byte formats and safe ordered actions; run.py
   records explicitly and normally replays without rewriting the oracle.
4. verify.py supplies positive/negative controls and defined compiled mutations;
   coverage.py separately measures routines/branches and ordinary differences.
5. REPORT.md records actual final identities, commands, local paths/hashes,
   findings and limits. Explicit-path Conventional Commits contain authored
   sources/contracts/reports only. Generated evidence stays ignored locally.
