use procs::display_fields;

#[derive(Debug)]
#[display_fields]
struct MyStruct {
    field1: i32,
    field2: String,
}
fn main() {
    let my_struct = MyStruct {
        field1: 42,
        field2: "Hello, World!".to_string(),
    };

    my_struct.display_fields();

}
