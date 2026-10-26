use super::*;

#[test]
fn byte_paths_pairing_aliases_and_recovery() {
    let root = std::env::temp_dir().join(format!(
        "tempo2-ah-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir(&root).unwrap();
    let mut stdout = vec![];
    let mut stderr = vec![];
    let mut input = std::io::empty();
    let mut io = OutputIo {
        stdout: &mut stdout,
        stderr: &mut stderr,
        stdin: &mut input,
        directory: &root,
    };
    let tokens = |a: &[&str]| a.iter().map(|s| s.as_bytes().to_vec()).collect::<Vec<_>>();
    for a in [
        &["t2", "-f"][..],
        &["t2", "-f", "p.par", "t.tim", "-outpar"][..],
    ] {
        assert!(get_inputs(tokens(a), &root, &mut io).is_err());
    }
    let c = get_inputs(
        tokens(&[
            "t2",
            "-f",
            "one.par",
            "one.tim",
            "-f",
            "two.par",
            "two.tim",
            "-pre",
            "-outpar",
            "answer.par",
        ]),
        &root,
        &mut io,
    )
    .unwrap();
    assert_eq!(c.par, vec![PathBuf::from("one.par"), "two.par".into()]);
    assert!(c.pre_only);
    assert_eq!(c.new_par, Some("answer.par".into()));
    let name = OsStr::new("space ; name.par");
    let par = root.join(name);
    std::fs::write(&par, []).unwrap();
    std::fs::write(par.with_extension("tim"), []).unwrap();
    let c = get_inputs(tokens(&["t2", "-allParTim"]), &root, &mut io).unwrap();
    assert_eq!(c.par[0].as_os_str(), name);
    let c = get_inputs(
        vec![
            b"t2".to_vec(),
            b"-f".to_vec(),
            b"\xff.par".to_vec(),
            b"\xff.tim".to_vec(),
        ],
        &root,
        &mut io,
    )
    .unwrap();
    assert_eq!(c.par[0].as_os_str().as_bytes(), b"\xff.par");
    let env = Environment {
        runtime: Some(root.clone()),
        ..Default::default()
    };
    for content in ["a a extra\n", "a b\nb a\n", "a\n"] {
        std::fs::write(root.join("alias.dat"), content).unwrap();
        assert!(expand(tokens(&["t2", "a"]), &env, &mut io).is_err());
    }
    std::fs::write(root.join("alias.dat"), "a b -pre\nb -f p.par p.tim\n").unwrap();
    let expanded = expand(tokens(&["t2", "a", "-newpar"]), &env, &mut io)
        .unwrap()
        .0;
    assert_eq!(
        expanded,
        tokens(&["t2", "-f", "p.par", "p.tim", "-pre", "-newpar"])
    );
    let long = "x".repeat(5000);
    std::fs::write(
        root.join("alias.dat"),
        format!("large -f {long}.par {long}.tim\n"),
    )
    .unwrap();
    assert_eq!(
        expand(tokens(&["t2", "large"]), &env, &mut io).unwrap().0[2].len(),
        5004
    );
    assert_eq!(
        set_plug_path(Some(OsStr::new("a::b:")), &root),
        vec!["a".into(), "".into(), "b".into(), root.join("plugins/")]
    );
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn enum_tables_include_source_markers_and_empty_sentinels() {
    assert_eq!(
        crate::core_output::LABEL_STR[..166],
        crate::tempo2::PARAMETER_NAMES
    );
    assert_eq!(crate::core_output::LABEL_STR[166], "");
    for (i, name) in crate::core_output::CONSTRAINT_STR[..68].iter().enumerate() {
        assert_eq!(
            *name,
            format!(
                "{:?}",
                crate::pulsar::Constraint::from_index(i as i32).unwrap()
            )
        );
    }
    assert_eq!(crate::core_output::CONSTRAINT_STR[68], "");
    assert_eq!(crate::core_output::SERIES_STR[3], "");
}
