// 4.?? Kleisli Categories Practices
type Writer<A> = (A, String);

// 4.1 - The Writer Categories
pub fn is_even(n: i64) -> (bool, String) {
    (n % 2 == 0, "is even".to_string())
}

pub fn negate(b: bool) -> (bool, String) {
    (!b, "Not so! ".to_string())
}

pub fn writer_id<A>(a: A) -> Writer<A> {
    (a, "".to_string())
}

// Self Challenge: How can you compose together the above functions above?
pub fn compose_writer<A, B, TypeIn, TypeOut, ANYTHING>(
    f: A,
    g: B,
) -> impl Fn(ANYTHING) -> Writer<TypeOut>
where
    A: Fn(ANYTHING) -> Writer<TypeIn>,
    B: Fn(TypeIn) -> Writer<TypeOut>,
{
    move |f_in| {
        let (first_op_result, logger) = f(f_in);
        let (second_op_result, logger_2) = g(first_op_result);
        (
            second_op_result,
            format!("{}{}", logger, logger_2).to_string(),
        )
    }
}

// Chapter 4 - Challeng 1
// Construct the Lkeisli category for partial functions (define composition and identity)
#[derive(Debug)]
pub enum Optional<A> {
    Some(A),
    None,
}

pub fn id_optional<A>(a: Optional<A>) -> Optional<A> {
    a
}

pub fn safe_root(a: f64) -> Optional<f64> {
    if a >= 0.0 {
        Optional::Some(f64::sqrt(a))
    } else {
        Optional::None
    }
}

pub fn safe_reciprocal(a: f64) -> Optional<f64> {
    if a != 0.0 {
        Optional::Some(1.0 / a)
    } else {
        Optional::None
    }
}

pub fn compose_optional<A_In, B_In, C>(
    a_func: impl Fn(A_In) -> Optional<B_In> + 'static,
    b_func: impl Fn(B_In) -> Optional<C> + 'static,
) -> impl Fn(A_In) -> Optional<C> + 'static {
    move |x: A_In| {
        let a_out = a_func(x);
        match a_out {
            Optional::Some(b_in) => match b_func(b_in) {
                Optional::Some(c) => Optional::Some(c),
                Optional::None => Optional::None,
            },
            Optional::None => Optional::None,
        }
    }
}
