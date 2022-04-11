macro_rules! my_vec {
    (
        $($element:expr),+
        $(,)?
    ) => {{
        let mut v = Vec::new();
        $(
            v.push($element);
        )+
        v
    }};
}
fn main() {
    let v = my_vec![1, 2, 3, 4];
    dbg!(v);
}
