use {
    super::{
        brute::{test_case, Spec},
        NoLazy, Ops,
    },
    rand::{distributions::Alphanumeric, prelude::StdRng, Rng, SeedableRng},
};

enum Cat {}
impl Ops for Cat {
    type Value = char;
    type Acc = String;
    fn proj(c: &char) -> String {
        c.to_string()
    }
    fn op(lhs: &String, rhs: &String) -> String {
        lhs.chars().chain(rhs.chars()).collect()
    }
}

fn random_value(rng: &mut StdRng) -> char {
    rng.sample(Alphanumeric) as char
}

#[test]
fn test_cat_typical_queries() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..20 {
        test_case::<NoLazy<Cat>, _>(
            &mut rng,
            random_value,
            &Spec {
                get: 4,
                fold: 2,
                push_back: 1,
                push_front: 1,
                insert: 1,
                pop_back: 1,
                pop_front: 1,
                delete: 1,
            },
        );
    }
}

#[test]
fn test_cat_insert_delete() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..20 {
        test_case::<NoLazy<Cat>, _>(
            &mut rng,
            random_value,
            &Spec {
                get: 4,
                fold: 2,
                insert: 1,
                delete: 2,
                ..Spec::default()
            },
        );
    }
}

#[test]
fn test_cat_push_pop() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..20 {
        test_case::<NoLazy<Cat>, _>(
            &mut rng,
            random_value,
            &Spec {
                get: 4,
                fold: 2,
                push_back: 2,
                push_front: 2,
                pop_back: 1,
                pop_front: 1,
                ..Spec::default()
            },
        );
    }
}

#[test]
fn test_affine_typical_queries_many_delete() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..20 {
        test_case::<NoLazy<Cat>, _>(
            &mut rng,
            random_value,
            &Spec {
                get: 4,
                fold: 2,
                push_back: 1,
                push_front: 1,
                insert: 1,
                pop_back: 2,
                pop_front: 2,
                delete: 2,
            },
        );
    }
}

#[test]
fn test_affine_typical_queries_many_push() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..20 {
        test_case::<NoLazy<Cat>, _>(
            &mut rng,
            random_value,
            &Spec {
                get: 4,
                fold: 2,
                push_back: 2,
                push_front: 2,
                insert: 2,
                pop_back: 1,
                pop_front: 1,
                delete: 1,
            },
        );
    }
}
