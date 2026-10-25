use crate::{observation::SourceObservation, pulsar_lifecycle::PulsarContext, source_tim};
use std::fs;

fn context() -> PulsarContext {
    let mut c = PulsarContext::unwritten(1);
    c.initialise(1, 0, 1).unwrap();
    c
}

#[test]
fn tim_writer_empty_backing_and_unwritten_boundary() {
    if !crate::source_par_tests::in_isolated_process(
        "source_tim_tests::tim_writer_empty_backing_and_unwritten_boundary",
    ) {
        return;
    }
    let dir = std::env::temp_dir().join(format!("tempo2-ac-writer-{}", std::process::id()));
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("empty.tim");
    let mut c = context();
    let view = c.adopt_observations(vec![SourceObservation::default()]);
    c.pulsars[0].obsn = Some(Some(view));
    assert!(
        source_tim::write_tim(&path, &mut c, 0, b"tempo2", &dir)
            .unwrap_err()
            .to_string()
            .contains("efac")
    );
    let o = &mut c.observations_mut(view).unwrap()[0];
    o.efac = Some(2.0);
    o.equad = Some(3.0);
    source_tim::write_tim(&path, &mut c, 0, b"tempo2", &dir).unwrap();
    assert_eq!(
        fs::read(&path).unwrap(),
        b"FORMAT 1\nEQUAD 3.000000\nEFAC 2.000000\n"
    );
    c.pulsars[0].obsn = Some(None);
    // Physical empty loops still need no initialized observation pointer.
    assert!(c.pulsar_observations_mut(0).unwrap().1.is_empty());
    assert!(
        source_tim::write_tim(&path, &mut c, 0, b"tempo2", &dir)
            .unwrap_err()
            .to_string()
            .contains("writeTim backing")
    );
    let input = dir.join("filtered.tim");
    fs::write(
        &input,
        b"FORMAT 1\nEFAC 2\nEQUAD 4\nFMIN 1500\nrow 1400 55000 3 pks\n",
    )
    .unwrap();
    let mut rows = vec![];
    source_tim::read_tim(
        &input,
        &mut c.pulsars[0],
        &mut rows,
        &mut 0,
        &mut 0,
        &dir,
        &mut vec![],
    )
    .unwrap();
    assert_eq!(c.pulsars[0].nobs, Some(0));
    let view = c.adopt_observations(rows);
    c.pulsars[0].obsn = Some(Some(view));
    source_tim::write_tim(&path, &mut c, 0, b"tempo2", &dir).unwrap();
    assert_eq!(
        fs::read(&path).unwrap(),
        b"FORMAT 1\nEQUAD 4.000000\nEFAC 2.000000\n"
    );
    fs::remove_dir_all(dir).unwrap();
}
