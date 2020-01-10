/// Convert a boolean to a feature:
pub fn boolf(x: bool) -> f32 {
    if x {
        1.0
    } else {
        -1.0
    }
}

/// Numerically stable sigmoid function
/// Source: http://timvieira.github.io/blog/post/2014/02/11/exp-normalize-trick/
/// Honestly not sure of primary source for this.
pub fn sigmoid(x: f32) -> f32 {
    if x < 0.0 {
        let a = x.exp();
        a / (1.0 + a)
    } else {
        1.0 / (1.0 + (-x).exp())
    }
}
