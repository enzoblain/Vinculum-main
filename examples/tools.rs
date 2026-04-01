use vinculum_hs::functions::tools::{addIfJust, addToAll, concatStrings, tryGetString};

#[vinculum_hs::main(haskell_directory = "examples/haskell")]
fn main() {
    let a = String::from("Hello");
    let b = String::from(" World");

    let result = concatStrings(a, b);
    println!("{}", result);

    let a = Some(10);
    let result = addIfJust(a, 5);
    println!("add_if_just(Some(10), 5) = {}", result);

    let result = addIfJust(None, 5);
    println!("add_if_just(None, 5) = {}", result);

    let result = tryGetString(42);
    println!("tryGetString(42) = {:?}", result);

    let result = tryGetString(0);
    println!("tryGetString(0) = {:?}", result);

    let v = vec![1, 2, 3];
    let result = addToAll(4, v);

    println!("{result:?}");
}
