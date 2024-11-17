use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    hash::{Hash, Hasher},
};

use chapter_4::{compose_optional, compose_writer, is_even, negate, safe_reciprocal, safe_root};
pub mod chapter_4;
pub mod chapter_5;

pub fn compose<A, A_Out, A_In, B, B_Out>(a: A, b: B) -> impl Fn(A_In) -> B_Out
where
    A: Fn(A_In) -> A_Out,
    B: Fn(A_Out) -> B_Out,
{
    move |a_in| b(a(a_in))
}

// 2.7 Challenges
// 1. HOF memoize

pub fn memoize<PureFunc, TypeIn, TypeOut>(func: PureFunc) -> impl FnMut(TypeIn) -> TypeOut
where
    PureFunc: Fn(TypeIn) -> TypeOut,
    TypeIn: Hash + Eq + Copy,
    TypeOut: Any + Clone,
{
    let mut map: HashMap<TypeIn, TypeOut> = HashMap::new();

    move |param: TypeIn| -> TypeOut {
        let entry = map.get(&param);
        match entry {
            Some(inner) => inner.clone(),
            None => {
                map.insert(param, func(param));
                map[&param].clone()
            }
        }
    }
}

fn main() {
    let add_five = |x: usize| x + 5;
    let add_three = |x: usize| x + 3;
    let complicated_math = |x: f64| x.powf(4382748.4892387487923);

    let add_eight = compose(add_three, add_five);

    println!("Hello, world! This is... {}", add_eight(0));

    // Memotest
    let mut memo_add_five = memoize(add_five);
    use std::time::Instant;
    let start = Instant::now();
    let answer = complicated_math(4.);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let answer = complicated_math(4.);
    let duration2 = start.elapsed();

    let start = Instant::now();
    let answer = complicated_math(4.);
    let duration3 = start.elapsed();

    println!("First call took: {:?}", duration1);
    println!("Second call took: {:?}", duration2);
    println!("Third call took: {:?}", duration3);
    println!("{}", answer);

    // 4.1 Examples
    let composed_func = compose_writer(is_even, negate);
    let x = composed_func(3);
    println!("{} - {}", x.0, x.1);

    // 4 Challenges
    let composed_func = compose_optional(safe_reciprocal, safe_root);
    let thingy = composed_func(0.0);
    println!("root by Zery: {:?}", thingy);

    let thingy = composed_func(14.0);
    println!("root by One: {:?}", thingy);

    let thingy = composed_func(-0.1);
    println!("root by -0.One: {:?}", thingy);
}
