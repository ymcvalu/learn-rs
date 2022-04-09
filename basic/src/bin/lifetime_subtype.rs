fn main() {}

struct Context<'s> {
    meta: &'s str,
}

/// we can declare a lifetime `'b` that lives at least as long as `'a`
/// by declaring `'b` using the syntax `'b: 'a`.
struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

/// the `T: 'a` syntax specifies that `T` can be any type implementing `Send`,
/// but if it contains any references, the references must live at least as long as `'a`.
struct Ref<'a, T: Send + 'a>(&'a T);

/// Adding a `'static` lifetime bound to `T` to constrain `T` to types that have only `'static` references or no any references.
struct StaticRef<T: 'static>(&'static T);
