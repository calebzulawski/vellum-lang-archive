vellum_lang::define! {
    impl mul a b = { std::ops::Mul::mul($a, $b) };
    impl add a b = { std::ops::Add::add($a, $b) };
//    let square x = mul x x;
//    let sqsum x y = add (square x) (square y);
}

#[test]
fn mul() {
    let value = tt_call::tt_call! {
        macro = [{ mul }]
        args = [{ 3, 4 }]
    };
    assert_eq!(value, 12);
}
