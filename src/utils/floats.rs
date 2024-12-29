pub const EPSILON: f64 = 1e-6;

macro_rules! approx_eq {
    ($left:expr, $right:expr) => {
        let (left, right) = ($left, $right);
        if (left - right).abs() > crate::utils::floats::EPSILON {
            panic!(
                r#"approx_eq failed: left: `{:?}`, right: `{:?}`"#,
                left, right
            );
        }
    };
}

pub(crate) use approx_eq;
