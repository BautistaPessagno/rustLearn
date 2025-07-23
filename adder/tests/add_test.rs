use adder::add;



//no hace falta usar el #[cfg(test)] porque ya está en el módulo de pruebas
#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}