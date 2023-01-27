use crate::math::modulo::Modulo;

type Mod = Modulo<11>;

#[test]
fn modulo_new() {
    check_modulo(Mod::new(17), 6);
}

#[test]
fn modulo_add() {
    let mut v = Mod::new(6);
    check_modulo(v + 7, 2);
    check_modulo(v + Mod::new(1), 7);
    v += 10;
    check_modulo(v, 5);
    v += Mod::new(1);
    check_modulo(v, 6);
}

#[test]
fn modulo_sub() {
    let mut v = Mod::new(6);
    check_modulo(v - 7, 10);
    check_modulo(v - Mod::new(1), 5);
    v -= 2;
    check_modulo(v, 4);
    v -= Mod::new(10);
    check_modulo(v, 5);
}

#[test]
fn modulo_mul() {
    let mut v = Mod::new(5);
    check_modulo(v * 7, 2);
    check_modulo(v * Mod::new(2), 10);
    v *= 3;
    check_modulo(v, 4);
    v *= Mod::new(10);
    check_modulo(v, 7);
}

#[test]
fn modulo_div() {
    let mut v = Mod::new(6);
    check_modulo(v / 3, 2);
    check_modulo(v / Mod::new(2), 3);
    v /= 6;
    check_modulo(v, 1);
}

#[test]
fn modulo_pow() {
    let v = Mod::new(7);
    check_modulo(v.pow(2), 5);
    check_modulo(v.pow(123121424123123123), 2);
}

#[test]
fn modulo_inv() {
    check_modulo(Mod::new(7).inv(), 8);
}

#[track_caller]
fn check_modulo<const MOD: u64>(actual: Modulo<MOD>, expected: u64) {
    assert_eq!(actual.val(), expected);
}
