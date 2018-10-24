use hvac::prelude::*;

#[test]
fn new_hvac_is_idle() {
    let mut hvac = Hvac::default();
    let state = hvac.tick(0);
    assert_eq!(state.service, None);
    assert_eq!(state.fan, false);
}

#[test]
fn new_hvac_enforces_min_heat_recover_constraints() {
    let mut hvac = Hvac::default().with_heat(None, Some(100));
    let state = hvac.heat();
    assert_eq!(state.service, None);
    assert_eq!(state.fan, false);
    for i in 0..100 {
        let state = hvac.tick(i);
        assert_eq!(state.service, None);
        assert_eq!(state.fan, false);
    }
    let state = hvac.tick(100);
    assert_eq!(state.service, Some(HvacService::Heat));
    assert_eq!(state.fan, true);
}

#[test]
fn new_hvac_enforces_min_cool_recover_constraints() {
    let mut hvac = Hvac::default().with_cool(None, Some(100));
    let state = hvac.cool();
    assert_eq!(state.service, None);
    assert_eq!(state.fan, false);
    for i in 0..100 {
        let state = hvac.tick(i);
        assert_eq!(state.service, None);
        assert_eq!(state.fan, false);
    }
    let state = hvac.tick(100);
    assert_eq!(state.service, Some(HvacService::Cool));
    assert_eq!(state.fan, true);
}

#[test]
fn new_hvac_enforces_min_fan_recover_constraints() {
    let mut hvac = Hvac::default().with_fan(None, Some(100));
    let state = hvac.fan_auto(false);
    assert_eq!(state.service, None);
    assert_eq!(state.fan, false);
    for i in 0..100 {
        let state = hvac.tick(i);
        assert_eq!(state.service, None);
        assert_eq!(state.fan, false);
    }
    let state = hvac.tick(100);
    assert_eq!(state.service, None);
    assert_eq!(state.fan, true);
}

#[test]
fn hvac_fan_auto_with_heat() {
    let mut hvac = Hvac::default().with_heat(None, None).with_fan(None, None);
    let state = hvac.heat();
    assert_eq!(state.service, Some(HvacService::Heat));
    assert_eq!(state.fan, true);
    let state = hvac.idle();
    assert_eq!(state.service, None);
    assert_eq!(state.fan, false);
}

#[test]
fn hvac_fan_auto_with_cool() {
    let mut hvac = Hvac::default().with_cool(None, None).with_fan(None, None);
    let state = hvac.cool();
    assert_eq!(state.service, Some(HvacService::Cool));
    assert_eq!(state.fan, true);
    let state = hvac.idle();
    assert_eq!(state.service, None);
    assert_eq!(state.fan, false);
}

#[test]
fn hvac_fan_auto_sequence() {
    let mut hvac = Hvac::default()
        .with_heat(None, None)
        .with_cool(None, None)
        .with_fan(None, None);
    let state = hvac.idle();
    assert_eq!(state.fan, false);
    let state = hvac.heat();
    assert_eq!(state.fan, true);
    let state = hvac.cool();
    assert_eq!(state.fan, true);
    let state = hvac.idle();
    assert_eq!(state.fan, false);
    let state = hvac.heat();
    assert_eq!(state.fan, true);
    let state = hvac.idle();
    assert_eq!(state.fan, false);
    let state = hvac.cool();
    assert_eq!(state.fan, true);
    let state = hvac.idle();
    assert_eq!(state.fan, false);
}

#[test]
fn hvac_fan_manual() {
    let mut hvac = Hvac::default()
        .with_heat(None, None)
        .with_cool(None, None)
        .with_fan(None, None);
    let state = hvac.fan_auto(false);
    assert_eq!(state.service, None);
    assert_eq!(state.fan, true);
    let state = hvac.heat();
    assert_eq!(state.service, Some(HvacService::Heat));
    assert_eq!(state.fan, true);
    let state = hvac.cool();
    assert_eq!(state.service, Some(HvacService::Cool));
    assert_eq!(state.fan, true);
    let state = hvac.idle();
    assert_eq!(state.service, None);
    assert_eq!(state.fan, true);
    let state = hvac.fan_auto(true);
    assert_eq!(state.service, None);
    assert_eq!(state.fan, false);
}

#[test]
fn fan_auto_min_run_carries_past_heat() {
    let mut hvac = Hvac::default()
        .with_heat(None, None)
        .with_fan(Some(1), None);
    let state = hvac.tick(0);
    assert_eq!(state.fan, false);
    let state = hvac.heat();
    assert_eq!(state.fan, true);
    let state = hvac.idle();
    assert_eq!(state.fan, true);
    let state = hvac.tick(1);
    assert_eq!(state.fan, false);
}
