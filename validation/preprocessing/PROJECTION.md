# Caller, field and action projection

The lossless state schema is fixed in probe.cc. `TYPES.json` contains every raw
struct declaration; initial ABI `layout` provides compiled offsets/extents for
all 95 observation fields and every projected pulsar field. `inventory.json`
provides complete field/parameter references for each primary routine. The table
below supplies the read/write and initialization interpretation of those lists.
`cases.json` preserves exact caller actions and file bytes; setup actions are
visible deltas, never silently rerun between native actions.

| State family | Read/write contract and caller ownership |
|---|---|
| Parameter backing | Real initialiseOne allocates all 2000 records / 9616 cells; source val/err/set/fit/labels retained. Caller assigns every prefit=17 and prefitErr=19. All six numeric/flag arrays, aSize, full link arrays/counts and all 100 bytes of each label are projected in enum/allocation order. Arrays are concatenated; aSize gives cell boundaries. |
| Wrapper parameter scan | Reads every shortlabel, aSize, link count and selected link target; writes val/prefit through real getter in ascending enum order, then fit/set flags and values. -nofit clears all ordinary parameter fit flags except START/FINISH. Unknown labels make no assignment; duplicate labels can write multiple records. |
| Units | Reads/writes val,err,prefit,prefitErr of the exact source-selected indices independently of paramSet. Reads fddi and TNChromIdx. All other cells, flags, labels, links and units control retained. xform_mjd reads prefit[0]; later BPJ iterations observe the already transformed element zero. |
| Epochs | Reads F0–F3, RA/DEC and motion fields, DM0–DM2, epochs and flags. Writes F0/F1, conditional F2, F0/F1 prefit, RA/DEC with their prefit/string forms, three epochs with prefit, and DM0 val only. F2 prefit, higher derivatives, DM prefit/errors and all other errors retained. |
| Observation backing | Entire 7680-byte records are caller-zeroed before explicit sentinels. All initialized unused rows and tails retained. SAT/BAT/frequency/error/phase/counters/deletion/tobs also have typed exports. Observation allocation capacity is separate from active nobs. |
| Stage2 observation writes | Four six-vector arrays reset, SAT adjusted in source order, deletion set by filters/DM, ordinary/FD membership appended, ordinary scale assigned only by stage2, -dm flag appended. Does not reset existing membership; deleted rows still undergo transformations. tobs/origErr and the fifth sun vector retain caller values unless the wrapper explicitly writes tobs. |
| Jumps / FD jumps | Caller initializes first eight values/errors/fit/index/SAT/scaled entries distinctly; all remaining backing is zero. nJumps/nfdJumps start at source zero. Jump strings are caller actions or FJUMP output. All full arrays/counters are projected, including index zero and unused tails. -nojump changes only nJumps; -nofit ordinary loop includes zero, FD starts at one. |
| Stage3 jump assignment | Reads fjumpID, undeleted matching-ID flag values; writes newly appended strings, observation memberships and fit flags. It retains jump values/errors/scales/FD indices. Application checks value only and includes deleted observations. |
| Phase jumps | Full longdouble times and integer IDs/directions/count projected. Caller adds bounded explicit times/IDs. Stage3 overwrites each ID with -1 before double-narrowed search from observation one; direction and time retained. |
| Time offsets | Full tOffset, f1/f2/t1/t2, site and flag arrays/count projected; initialized by caller actions. Read-only controls; ordered strict-bound tests use already modified SAT and actual alias lookup. |
| Selection | Reads active observation SAT/freq/toaErr/flags and prior deletion state for every requested pulsar; writes deletion only. Source feof/token state and in-place matching order remain local native execution. No caller reset between lines/calls. |
| White/T2/TN noise | Full matching IDs/values, values/counts and global factors are projected. White-file and wrapper T2/TN transformations write toaErr, never origErr. TNSECORR reads every -tobs flag and writes tobs; fixtures assign at least one to every active row. TNSQ's independently initialized zero denominator is defined nonfinite arithmetic. |
| Stage1 controls | Source reads/writes EPHVER and wave values, START based on last stored SAT, nWhite and nWhite_dm (latter retained), tempo1/units/timeEphemeris/dilateFreq/planetShapiro/t2cMethod/correctTroposphere/ne_sw. Emulation affects global obliquity; every call resets that global first. |
| Wrapper strings/control | name, clock, whiteNoiseModelFile, filterStr/passStr/fjumpID, nits, dm_series_type and all associated counters are projected. File names/argv are declared external inputs. Global PAR runs only assigned scalar/name/NITS grammar; its real parameter backing mutations are projected. |
| Globals and external state | Obliquity, forceGlobalFit, plugin paths/machine/count, diagnostic counters/full storage, debug/quiet/CVS/MAX values projected. Plugin static count and observatory alias cache are observed through repeated real calls and exact streams; their private implementation storage is not read through an invasive accessor. File contents, path order and fresh-process/repeated-call boundaries are explicit inputs. |

Initialization and teardown are harness responsibilities. The process owns its
allocations until termination, including the atexit snapshot. It intentionally
does not call destroyOne before that snapshot or turn freed storage into evidence.
This is not a leak test. Missing/open-file failures are explicit source exits;
unchecked DM fopen, first failed read of an empty file, missing argv/scan values,
invalid conversions, unchecked overflow and uninitialized ndm/TOBS/module paths
stay unrun as cataloged in inventory.json.

The callback's initialized state changes are declared external input behavior.
Its implementation neither replaces a core function nor makes a physical noise
model claim. Its binary/source/defines are recorded in each executed profile.
