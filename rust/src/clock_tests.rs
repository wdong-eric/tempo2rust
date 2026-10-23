//! Assignment L observation harness, never a second clock implementation.
//! Fresh processes drive the production process session; no reset entry point.
use crate::{
    PRECISION_BITS, TimingError,
    clkcorr::{self, ClockSession},
    clock_diagnostics::boundary,
    clock_text,
    observation::{format_string, read_string},
    tabulated_function::TabulatedFunction,
    tempo2::SourceObservation,
    toa2utc::{ClockPulsar, toa2utc},
};
use rug::Float;
use serde_json::{Value, json};
use std::{fs, path::Path};
// L's explicit caller state; SourceObservation::default remains wholly unwritten.
fn caller() -> SourceObservation {
    let mut o = SourceObservation::default();
    o.seed_clock_caller();
    o.sat = Some(Float::with_val(PRECISION_BITS, 0));
    o.telID.fill(Some(0));
    o.clockCorr = Some(0);
    o
}
fn number(x: f64) -> Value {
    if x.is_nan() {
        json!({"class":"NaN"})
    } else if x.is_infinite() {
        json!({"class":if x>0.0{"+Infinity"}else{"-Infinity"}})
    } else {
        json!({"class":"finite","decimal":x.to_string(),"bits":format!("{:016x}",x.to_bits())})
    }
}
fn weight(x: f32) -> Value {
    let mut v = json!({"value":number(x as f64)});
    if x.is_finite() {
        v["binary32"] = json!(format!("{:08x}", x.to_bits()));
    }
    v
}
fn text(s: &[u8]) -> String {
    String::from_utf8(s.to_vec()).expect("probe JSON fixture text must be valid UTF-8")
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn table_state(t: &TabulatedFunction) -> Value {
    json!({"name":text(&t.name),"header":text(&t.header),"start":number(t.start().unwrap()),"end":number(t.end().unwrap()),"samples":t.samples.iter().map(|s|vec![number(s[0]),number(s[1])]).collect::<Vec<_>>()})
}
fn state(s: &ClockSession) -> Value {
    json!({"initialized":s.initialized,"functions":s.functions.iter().map(|f|{
        let mut v=json!({"path":f.path,"name":text(&f.name),"from":text(&f.from),"to":text(&f.to),"badness":weight(f.badness),"loaded":f.loaded,"start":number(f.start),"end":number(f.end)});
        if f.loaded{v["samples"]=json!(f.table.samples.len());v["header"]=json!(text(&f.table.header));}v
    }).collect::<Vec<_>>(),"sequences":s.sequences.iter().map(|q|json!({"from":text(&q.from),"to":text(&q.to),"from_key":text(&q.from_key),"to_key":text(&q.to_key),"start":number(q.start),"end":number(q.end),"indices":q.indices})).collect::<Vec<_>>(),"warnings":s.diagnostics.warnings,"errors":s.diagnostics.errors})
}
fn obs_state(o: &SourceObservation, before: Option<&SourceObservation>) -> Value {
    json!({"sat":o.sat.as_ref().unwrap().to_string_radix(10,Some(40)),"sat_binary64":number(o.sat.as_ref().unwrap().to_f64()),"telID":text(&read_string(&o.telID, "telID").unwrap()),"clockCorr":o.clockCorr,"count":o.nclock_correction,"slots":o.correctionsTT.iter().enumerate().map(|(i,c)|{
        let mut v=json!({"value":number(c.correction.unwrap()),"destination_bytes":hex(&c.corrects_to.map(Option::unwrap))});
        if let Some(before)=before{
            let old=&before.correctionsTT[i];
            let current=[c.correction.unwrap().to_le_bytes().as_slice(),&c.corrects_to.map(Option::unwrap)].concat();
            let prior=[old.correction.unwrap().to_le_bytes().as_slice(),&old.corrects_to.map(Option::unwrap)].concat();
            let mask:String=current.iter().zip(prior).map(|(a,b)|if *a==b{'0'}else{'1'}).collect();v["changed_bytes"]=json!(mask);
        }v
    }).collect::<Vec<_>>()})
}
fn numeric(s: &str) -> f64 {
    clock_text::number_prefix(s.as_bytes()).unwrap().0
}
fn nullable(s: &str) -> Option<&[u8]> {
    if s == "@null" {
        None
    } else {
        Some(s.as_bytes())
    }
}
#[test]
#[ignore = "fresh-process clock comparison probe; driven by validation/clocks_port/run.py"]
fn probe() {
    let input = std::env::var("CLOCK_INPUT").unwrap();
    let output = std::env::var("CLOCK_OUTPUT").unwrap();
    let actions: Vec<Vec<String>> = serde_json::from_slice(&fs::read(&input).unwrap()).unwrap();
    let mut session = clkcorr::test_session();
    session.diagnostics.source_directory =
        std::env::var("CLOCK_SOURCE_LOCATION").unwrap().into_bytes();
    let mut observations = vec![vec![caller(); 3]; 2];
    let mut pulsars = vec![ClockPulsar::default(), ClockPulsar::default()];
    let mut table = TabulatedFunction::default();
    let mut events = Vec::new();
    let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
    let mut failure = None;
    for (event, a) in actions.iter().enumerate() {
        let op = a[0].as_str();
        let index = |i: usize| a[i].parse::<usize>().unwrap();
        let integer = |i: usize| a[i].parse::<i32>().unwrap();
        let begin = format!("@BEGIN {event} {op}\n");
        stdout.extend(begin.as_bytes());
        stderr.extend(begin.as_bytes());
        events.push(json!({"event":event,"phase":"begin","operation":op}));
        session.queries.clear();
        let mut direct_queries = Vec::new();
        let result:Result<Value,TimingError>=session.run(|s|{
            let mut result=json!({});
            match op{
                "glob"=>{use std::os::unix::ffi::OsStrExt;result=json!({"paths_hex":crate::clock_glob::files(a[1].as_bytes()).iter().map(|p|hex(p.as_os_str().as_bytes())).collect::<Vec<_>>()});},
                "profile"=>result=json!({"float_digits":24,"double_digits":53,"longdouble_digits":PRECISION_BITS,"longdouble_size":0,"clock_capacity":30,"table_size":0,"observation_size":0}),
                "env"=>{s.root=if a[1]=="@unset"{None}else{Some(a[1].clone().into())};s.extra=a[2].as_bytes().to_vec();},
                "index"=>s.ensure_initialized(integer(1))?,
                "index_file"=>s.add_indexed_clock_file(Path::new(&a[1]),integer(2))?,
                "obs"=>{
                    let o=&mut observations[index(1)/3][index(1)%3];*o=caller();
                    o.sat=Some(Float::with_val(PRECISION_BITS,Float::parse(&a[2]).unwrap()));format_string(&mut o.telID,a[3].as_bytes());o.clockCorr=Some(integer(4));o.nclock_correction=Some(integer(5));
                    for(i,c)in o.correctionsTT.iter_mut().enumerate(){c.correction=Some(900.0+i as f64);c.corrects_to.fill(Some(b'~'));format_string(&mut c.corrects_to[..31],format!("sentinel_{i:02}").as_bytes());}
                    result["observation"]=obs_state(o,None);
                },
                "slot"=>{let o=&mut observations[index(1)/3][index(1)%3];let c=&mut o.correctionsTT[index(2)];c.correction=Some(numeric(&a[3]));let bytes=a[4].as_bytes();if bytes.len()>=32{return Err(boundary("clock_correction_string_storage"));}format_string(&mut c.corrects_to,bytes);},
                "load"=>{table.load(Path::new(&a[1]),&mut s.diagnostics)?;result["table"]=table_state(&table);},
                "value"|"empty_value"=>{let x=if op=="value"{numeric(&a[1])}else{1.0};let value=table.get_value(x,&mut s.diagnostics)?;if op=="value"{result["query"]=number(x);}result["value"]=number(value);direct_queries.push(json!({"name":text(&table.name),"epoch":number(x),"value":number(value)}));},
                "empty_start"=>result["value"]=number(table.start()?),"empty_end"=>result["value"]=number(table.end()?),
                "define"=>s.define_sequence(nullable(&a[1]),integer(2))?,
                "clock"|"correction"|"tt"=>{
                    let o=&mut observations[index(1)/3][index(1)%3];let before=o.clone();
                    match op{
                        "clock"=>s.get_clock_corrections(o,nullable(&a[2]),nullable(&a[3]),integer(4))?,
                        "correction"=>result["value"]=number(s.get_correction(o,nullable(&a[2]),nullable(&a[3]),integer(4))?),
                        _=>result["value"]=number(clkcorr::get_correction_tt(o)?),
                    }
                    result["before"]=obs_state(&before,None);result["after"]=obs_state(o,Some(&before));
                },
                "path"=>{s.ensure_initialized(0)?;let p=s.build_directed_path(a[1].as_bytes(),a[2].as_bytes(),numeric(&a[3]))?;result=json!({"source_present":p.source_present,"target_present":p.target_present,"reverse_hint":p.reverse_hint,"indices":p.indices});},
                "helper_fill"|"helper_accumulate"=>{
                    let o=&mut observations[index(1)/3][index(1)%3];let before=o.clone();let mut total=numeric(&a[5]);
                    let ok=if op=="helper_fill"{s.fill_corrections(o,index(2),a[4].as_bytes(),a[3].as_bytes(),&mut total)?}else{s.accumulate_correction(o,index(2),a[4].as_bytes(),a[3].as_bytes(),&mut total)?};
                    result=json!({"ok":ok,"value":number(total),"before":obs_state(&before,None),"after":obs_state(o,Some(&before))});
                },
                "pulsar"=>{let p=&mut pulsars[index(1)];p.nobs=integer(2);p.njumps=integer(3);p.from=a[4].as_bytes().to_vec();p.to=a[5].as_bytes().to_vec();p.no_warnings=integer(6);},
                "jump"=>{let p=&mut pulsars[index(1)];let i=index(2);p.jump_value[i]=numeric(&a[3]);p.jump_sat[i]=integer(4) as i8;p.jump_string[i]=a[5].as_bytes().to_vec();},
                "members"=>{let o=&mut observations[index(1)/3][index(1)%3];o.obsNjump=Some((a.len()-2) as i32);for(i,x)in a[2..].iter().enumerate(){*o.jump.get_mut(i).ok_or_else(||boundary("clock_jump_storage"))?=Some(x.parse().unwrap());}},
                "toa"=>{
                    let before=observations.clone();toa2utc(s,&pulsars,&mut observations,integer(1))?;let mut rows=Vec::new();
                    for p in 0..index(1){for i in 0..pulsars[p].nobs as usize{rows.push(json!({"index":p*3+i,"before":obs_state(&before[p][i],None),"after":obs_state(&observations[p][i],Some(&before[p][i]))}));}}
                    result["observations"]=json!(rows);
                },
                "copy"=>{fs::copy(&a[1],&a[2]).unwrap();},"replace"=>{fs::rename(&a[1],&a[2]).unwrap();},"remove"=>{fs::remove_file(&a[1]).unwrap();},
                "debug"=>{s.diagnostics.debug=integer(1)!=0;s.diagnostics.display_version=integer(2)==1;},
                _=>panic!("unknown operation {op}"),
            }Ok(result)
        });
        for (is_stderr, bytes) in session.diagnostics.events.drain(..) {
            if is_stderr {
                stderr.extend(bytes);
            } else {
                stdout.extend(bytes);
            }
        }
        match result {
            Ok(result) => {
                let mut queries:Vec<_>=session.queries.iter().map(|q|json!({"name":text(&q.name),"epoch":number(q.epoch),"value":number(q.value)})).collect();
                queries.extend(direct_queries);
                events.push(json!({"event":event,"phase":"end","operation":op,"result":result,"clock_state":state(&session),"table_queries":queries}));
                let end = format!("@END {event} {op}\n");
                stdout.extend(end.as_bytes());
                stderr.extend(end.as_bytes());
            }
            Err(error) => {
                // Inspect defined prefix separately, never fabricate a completed
                // source event. Reentry must preserve the terminal owner/session.
                let again = session.run(|_| Ok(())).unwrap_err();
                assert_eq!(error.field, again.field);
                assert_eq!(error.message, again.message);
                failure = Some(
                    json!({"field":error.field,"cause":error.message,"event":event,"clock_state":state(&session),"observations":observations.iter().flat_map(|p|p.iter().map(|o|obs_state(o,None))).collect::<Vec<_>>(),"table_header_initialized_bytes":hex(&table.header_storage),"table_samples":table.samples.iter().map(|s|vec![number(s[0]),number(s[1])]).collect::<Vec<_>>(),"reentry":"same terminal state"}),
                );
                break;
            }
        }
    }
    let logs = json!({"keys":session.diagnostics.keys.iter().map(|x|text(x)).collect::<Vec<_>>(),"warnings":session.diagnostics.warning_log.iter().map(|x|text(x)).collect::<Vec<_>>(),"errors":session.diagnostics.error_log.iter().map(|x|text(x)).collect::<Vec<_>>(),"warned_multiple":session.diagnostics.warned_multiple});
    use sha2::{Digest, Sha256};
    let input_sha256 = format!("{:x}", Sha256::digest(fs::read(&input).unwrap()));
    fs::write(output,serde_json::to_vec(&json!({"case_id":std::env::var("CLOCK_CASE_ID").unwrap(),"run_token":std::env::var("CLOCK_RUN_TOKEN").unwrap(),"input_sha256":input_sha256,"events":events,"stdout":text(&stdout),"stderr":text(&stderr),"returncode":if failure.is_some(){1}else{0},"failure":failure,"logs":logs})).unwrap()).unwrap();
}

#[test]
fn byte_parsing_and_operation_local_table_boundaries() {
    use crate::clock_diagnostics::Diagnostics;
    let mut diagnostics = Diagnostics::default();
    let path = Path::new("source.clk");
    let t = TabulatedFunction::from_bytes(
        path,
        b"# A\xff B\0hidden\n1 10junk\n2 0x1p2\n",
        &mut diagnostics,
    )
    .unwrap();
    assert_eq!(t.header, b"# A\xff B");
    assert_eq!(t.samples, vec![[1.0, 10.0], [2.0, 4.0]]);
    assert_eq!(
        clkcorr::normalize_clock_name(b" \t\xffa\x0b\r\n"),
        b"\xffA\x0b"
    );
    let mut empty = TabulatedFunction::default();
    assert_eq!(empty.start().unwrap_err().field, "table_sample_storage");
    assert_eq!(
        empty.load(path, &mut diagnostics).unwrap_err().field,
        "table_sample_storage"
    );
    let data = [b"#\0".as_slice(), &[b'x'; 254], b"\n1 2\n2 4\n"].concat();
    let error = TabulatedFunction::from_bytes(path, &data, &mut diagnostics).unwrap_err();
    assert_eq!(error.field, "table_header_storage"); // bytes read, not visible strlen
    assert_eq!(
        clock_text::header(b"# A B 0x1p-2"),
        Some((b"A".to_vec(), b"B".to_vec(), 0.25))
    );
}

#[test]
fn diagnostic_keys_counters_storage_and_suppression() {
    use crate::clock_diagnostics::Diagnostics;
    let mut d = Diagnostics::default();
    d.display(1, b"K1", b"body", b"x", 2).unwrap();
    assert_eq!(d.warnings, 0);
    for _ in 0..3 {
        d.display(1, b"K1", b"body", b"x", 0).unwrap();
    }
    assert_eq!(d.warnings, 2);
    assert_eq!(d.keys, vec![b"K1".to_vec()]);
    assert!(d.warned_multiple);
    d.display(3, b"minor", b"body", b"", 0).unwrap();
    assert_eq!(d.warnings, 2);
    d.display(1, b"K1", b"body", b"x", -1).unwrap();
    assert_eq!(d.warnings, 2);
    assert!(d.events.last().unwrap().1.starts_with(b"[K1] *"));
    for _ in 0..35 {
        d.display(2, b"E", &[b'x'; 400], b"", 2).unwrap();
    }
    assert_eq!(d.errors, 35);
    assert_eq!(d.error_log.len(), 30);
    assert_eq!(d.error_log[0].len(), 359);
    while d.keys.len() < 50 {
        let key = format!("K{}", d.keys.len());
        d.display(1, key.as_bytes(), b"body", b"", 0).unwrap();
    }
    d.display(1, b"full", b"body", b"", 0).unwrap();
    assert_eq!(d.errors, 36);
    assert_eq!(d.keys.len(), 50);
    assert_eq!(d.warning_log.len(), 30);
}

#[test]
#[ignore = "process-lifetime provenance control; fresh process via L verifier"]
fn process_provenance() {
    use sha2::{Digest, Sha256};
    let work = std::path::PathBuf::from(std::env::var("CLOCK_WORK").unwrap());
    let a = work.join("root-a");
    let b = work.join("root-b");
    for root in [&a, &b] {
        fs::create_dir_all(root.join("clock")).unwrap();
    }
    let path = a.join("clock/edge.clk");
    let original = b"# A B\n0 1\n10 1\n";
    let replacement = b"# X Y\n0 7\n10 7\n";
    fs::write(&path, original).unwrap();
    fs::write(b.join("clock/edge.clk"), b"# A B\n0 99\n10 99\n").unwrap();
    let mut obs = SourceObservation {
        sat: Some(Float::with_val(PRECISION_BITS, 5)),
        ..caller()
    };
    clkcorr::with_session(&a, |s| {
        s.get_clock_corrections(&mut obs, Some(b"A"), Some(b" a "), 0)?;
        assert!(!s.initialized);
        s.ensure_initialized(0)
    })
    .unwrap();
    fs::write(&path, replacement).unwrap();
    let indexed = format!("{:x}", Sha256::digest(original));
    let loaded = format!("{:x}", Sha256::digest(replacement));
    let mut records = Vec::new();
    clkcorr::with_session(&a, |s| {
        s.get_clock_corrections(&mut obs, Some(b"A"), Some(b"B"), 1)?;
        assert_eq!(clkcorr::get_correction_tt(&obs)?, 7.0);
        assert_eq!(s.functions[0].indexed.sha256, indexed);
        assert_eq!(s.functions[0].table.sha256, loaded);
        assert_eq!(s.functions[0].from, b"A");
        assert_eq!(s.functions[0].table.header, b"# X Y\n");
        s.export_resources(&a, &mut records);
        Ok(())
    })
    .unwrap();
    assert_eq!(records[0].path, "clock/edge.clk");
    assert_eq!(records[0].sha256, loaded);
    fs::remove_file(&path).unwrap();
    records.clear();
    clkcorr::with_session(&b, |s| {
        s.get_clock_corrections(&mut obs, Some(b"A"), Some(b"B"), 1)?;
        assert_eq!(clkcorr::get_correction_tt(&obs)?, 7.0);
        s.export_resources(&b, &mut records);
        assert_eq!(s.functions.len(), 1);
        Ok(())
    })
    .unwrap();
    assert_eq!(records[0].path, path.to_string_lossy());
    assert_eq!(records[0].sha256, loaded);
    fs::write(std::env::var("CLOCK_PROVENANCE_OUTPUT").unwrap(),serde_json::to_vec_pretty(&json!({"identity_before_index":true,"indexed_sha256":indexed,"sample_sha256":loaded,"after_removal_changed_root":records,"value_s":7,"index_count":1,"status":"PASS"})).unwrap()).unwrap();
}
