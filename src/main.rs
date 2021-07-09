include!(concat!(env!("OUT_DIR"), "/example.rs"));

fn main() {
    println!(
        "{:?}",
        Example {
            example_field: "foo".to_owned()
        }
    );
}
