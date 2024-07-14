const SIMPLE: &str = r#"
let P(x) = 3*x + 1;
let Q(x) = P@5 * P * x;
let result = P(1) + P(2);
"#;
