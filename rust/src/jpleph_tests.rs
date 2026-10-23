//! I-only observation driver corresponding to validation/jpl/probe.c commands.
//! Uses production Rust functions; control/fault commands are explicit test inputs.
use super::*;
use crate::stdio::OBSERVER;
use serde_json::{Value, json};
use std::{fs, io::{Read, Seek, SeekFrom, Write}, os::unix::fs::MetadataExt};

fn number(x: f64) -> Value {
    if x.is_finite() { json!({"class":"finite","decimal":x.to_string(),"bits":format!("{:016x}",x.to_bits())}) }
    else { let c = if x.is_nan() {"NaN"} else if x.is_sign_negative() {"-Infinity"} else {"+Infinity"}; json!({"class":c,"decimal":c}) }
}
fn numbers(p: &[f64]) -> Value { Value::Array(p.iter().map(|x| number(*x)).collect()) }
fn bytes(p: &[u8]) -> String { p.iter().map(|x| format!("{x:02x}")).collect() }
fn fill(p: &mut [f64]) { for (i,x) in p.iter_mut().enumerate() { *x = -9000.0-i as f64; } }
fn snapshot(e: &Ephemeris, controlled: bool, v: &mut Value) {
    v["cache_loc"] = json!(e.curr_cache_loc);
    v["cache_bytes"] = json!(bytes(&e.cache));
    if controlled {
        v["sun_epoch"] = number(e.pvsun_t.unwrap());
        v["sun"] = numbers(e.pvsun().unwrap());
        v["n_posn"] = json!(e.iinfo.n_posn_avail);
        v["n_vel"] = json!(e.iinfo.n_vel_avail);
        v["twot"] = number(e.iinfo.twot);
        v["posn_coeff"] = numbers(&e.iinfo.posn_coeff);
        v["vel_coeff"] = numbers(&e.iinfo.vel_coeff);
    }
}
fn header(e: &Ephemeris, v: &mut Value) {
    v["header_doubles"] = numbers(&[e.ephem_start,e.ephem_end,e.ephem_step,e.au,e.emrat]);
    for (key,n) in [("ncon",e.ncon),("version",e.ephemeris_version),("kernel_size",e.kernel_size),
        ("recsize",e.recsize),("ncoeff",e.ncoeff),("swap",u32::from(e.swap_bytes))] { v[key] = json!(n); }
    v["ipt"] = json!(e.ipt);
}

#[test]
#[ignore = "executed in fresh processes by validation/jpl_port/run.py"]
fn probe() {
    OBSERVER.with(|o| *o.borrow_mut() = Default::default());
    init_error(-9);
    let commands = fs::read_to_string(std::env::var("JPL_COMMANDS").unwrap()).unwrap();
    let mut events = vec![json!({"op":"startup","init_error":jpl_init_error_code()})];
    let mut eph: Option<Ephemeris> = None;
    let mut controlled = false;
    for (event,line) in commands.lines().enumerate() {
        let args: Vec<_> = line.split_whitespace().collect();
        let op = args[0];
        let mut v = json!({"event":event,"op":op});
        let result: Result<()> = (|| {
            match op {
                "open" => {
                    let flags: u32 = args[2].parse().unwrap();
                    let mut names = vec![b'?';2048*6];
                    let mut values = [0.0;2048]; fill(&mut values);
                    let mut values: Vec<u8> = values.iter().flat_map(|x| x.to_ne_bytes()).collect();
                    // Exercise the same-descriptor provenance pass on every H open.
                    let file = File::open(args[1]).and_then(|mut f| {
                        let mut observed = Vec::new(); f.read_to_end(&mut observed)?;
                        f.seek(SeekFrom::Start(0))?; Ok(f)
                    });
                    eph = Ephemeris::init(file, if flags&1 != 0 {Some(&mut names)} else {None},
                        if flags&2 != 0 {Some(&mut values)} else {None})?;
                    controlled = false;
                    v["nonnull"] = json!(eph.is_some());
                    v["init_error"] = json!(jpl_init_error_code());
                    if let Some(e) = &eph {
                        header(e,&mut v);
                        if flags&1 != 0 { v["name_bytes"] = json!(bytes(&names[..e.ncon as usize*6])); }
                        if flags&2 != 0 { v["constants"] = numbers(&(0..e.ncon as usize).map(|i| double(&values,i*8,false)).collect::<Vec<_>>()); }
                    }
                }
                "control" => {
                    let e = eph.as_mut().unwrap();
                    e.pvsun_t = Some(f64::NAN);
                    let mut sun = [0.0;9]; fill(&mut sun); e.pvsun = Some(sun);
                    e.iinfo.twot = 0.0; e.iinfo.n_posn_avail = 2; e.iinfo.n_vel_avail = 2;
                    e.iinfo.posn_coeff[2..].fill(0.0); e.iinfo.vel_coeff[2..].fill(0.0);
                    controlled = true; snapshot(e,controlled,&mut v);
                }
                "state" | "state_nut4" => {
                    let e = eph.as_mut().unwrap();
                    let mut pv = [0.0;96]; fill(&mut pv);
                    let mut nut = [0.0;6]; fill(&mut nut);
                    let mut list = [0;14];
                    let (et,bary) = if op == "state" {
                        for i in 0..14 { list[i] = args[3+i].parse().unwrap(); }
                        (args[1].parse().unwrap(),args[2].parse().unwrap())
                    } else { list[11] = 2; (e.ephem_start+1.0,1) };
                    let n = if op == "state_nut4" {4} else {6};
                    let status = e.state(et,&list,&mut pv.chunks_mut(6).collect::<Vec<_>>(),&mut nut[..n],bary)?;
                    v["status"] = json!(status);
                    if op == "state" { v["pv"] = numbers(&pv); v["nut"] = numbers(&nut); snapshot(e,controlled,&mut v); }
                }
                "pleph" | "pleph_status" => {
                    let e = eph.as_mut().unwrap();
                    let mut out = [0.0;6]; fill(&mut out);
                    let s = e.pleph(args[1].parse().unwrap(),args[2].parse().unwrap(),args[3].parse().unwrap(),&mut out,args[4].parse().unwrap())?;
                    v["status"] = json!(s);
                    if op == "pleph" && s == 0 { v["out"] = numbers(&out); }
                    if controlled { snapshot(e,controlled,&mut v); }
                }
                "constant" => {
                    let mut name = [b'?';7];
                    let value = eph.as_mut().unwrap().get_constant(args[1].parse().unwrap(),&mut name)?;
                    v["name_bytes"] = json!(bytes(&name)); v["value"] = number(value);
                }
                "getter" => {
                    let e = eph.as_ref().unwrap(); let offset = args[2].parse().unwrap();
                    v["value"] = if args[1] == "d" {number(e.get_double(offset)?)} else {json!(e.get_long(offset)?)};
                }
                "fault" => OBSERVER.with(|o| o.borrow_mut().fail[match args[1] {"seek"=>0,"read"=>1,"alloc"=>2,_=>panic!()}] = args[2].parse().unwrap()),
                "truncate" => fs::OpenOptions::new().write(true).open(args[1]).unwrap().set_len(args[2].parse().unwrap()).unwrap(),
                "replace" => fs::rename(args[2],args[1]).unwrap(),
                "overwrite" => {
                    let data = fs::read(args[2]).unwrap();
                    let mut f = fs::OpenOptions::new().write(true).open(args[1]).unwrap();
                    f.write_all(&data).unwrap(); f.set_len(data.len() as u64).unwrap();
                }
                "close" => { eph = None; controlled = false; v["init_error"] = json!(jpl_init_error_code()); }
                "interp" => {
                    let mut iinfo = Interpolation::new();
                    let coef: Vec<u8> = (0..432).flat_map(|i| ((if i%7 == 0 {-1.0} else {1.0})*(i+1) as f64/16.0).to_ne_bytes()).collect();
                    let mut out = [0.0;9]; fill(&mut out);
                    iinfo.interp(&coef,[args[1].parse().unwrap(),args[2].parse().unwrap()],args[3].parse().unwrap(),args[4].parse().unwrap(),args[5].parse().unwrap(),args[6].parse().unwrap(),&mut out)?;
                    v["out"] = numbers(&out);
                }
                "helpers" => {
                    let mut b: Vec<u8> = (0..16).collect();
                    swap_32_bit_val((&mut b[..4]).try_into().unwrap()); swap_64_bit_val(&mut b)?;
                    v["swap_bytes"] = json!(bytes(&b));
                    v["dimensions"] = json!((0..16).map(dimension).collect::<Vec<_>>());
                }
                _ => panic!("unknown {op}"),
            }
            if let Some(e) = &eph && ["open","truncate","replace","overwrite"].contains(&op) {
                let handle = e.stream.metadata_file.metadata().unwrap();
                let path = fs::metadata(args[1]).unwrap();
                v["path_matches_open_inode"] = json!(handle.dev() == path.dev() && handle.ino() == path.ino());
                v["open_file_size"] = json!(handle.len()); v["path_file_size"] = json!(path.len());
            }
            Ok(())
        })();
        if let Err(e) = result { v["boundary"] = json!(e.0); }
        OBSERVER.with(|o| { let o = o.borrow(); v["io_calls"] = json!(o.calls); v["last_fread_size_requested_returned"] = json!(o.last_read); });
        let stopped = v.get("boundary").is_some(); events.push(v);
        if stopped { break; }
    }
    fs::write(std::env::var("JPL_OUTPUT").unwrap(),serde_json::to_vec_pretty(&events).unwrap()).unwrap();
}
