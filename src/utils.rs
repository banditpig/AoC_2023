use load_file::load_str;

pub fn load_input(file: &str) -> Vec<&'static str> {
    let v = load_str!(file)
        .split('\n')
        .collect::<Vec<&str>>();
    v
}