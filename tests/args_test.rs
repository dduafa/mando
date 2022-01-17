use crate::args;

#[test]
fn it_can_create_a_new_args_struct() {
    let params: Vec<String> = vec![];
    assert_eq!(4, args::Args::new(params));
}
