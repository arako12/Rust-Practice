use rust_dependency1::greet;

#[test]
fn integration_greet_works() {
    assert_eq!(greet(), "Hello, test!");
}