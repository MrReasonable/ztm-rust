// Topic: Basic macro repetitions
//
// Requirements:
//   * Create a macro to generate hashmaps.
//   * The macro must be able to accept multiple key/value pairs.
//   * Print out the generated hashmap using the `dbg!` macro to ensure it works.

macro_rules! hashmap {
    (
        $($key:expr => $val:expr),+
        $(,)?
    ) => {{
        let mut hm = std::collections::HashMap::new();
        $(
            hm.insert($key, $val);
        )+
        hm
    }};
}

fn main() {
    let hm = hashmap! {"ian"=>32, "bob"=>13, "lucy"=>15, "michael"=>150};
    dbg!(hm);
}
