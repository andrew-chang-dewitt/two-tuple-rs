trait Append<T> {
    type Result;

    fn append(self, other: T) -> Self::Result;
}

impl<Other> Append<Other> for () {
    type Result = Other;

    fn append(self, other: Other) -> Self::Result {
        other
    }
}

impl<Head, Tail, Other> Append<Other> for (Head, Tail)
where
    Tail: Append<Other>,
{
    type Result = (Head, <Tail as Append<Other>>::Result);

    fn append(self, other: Other) -> Self::Result {
        let (head, tail) = self;
        (head, tail.append(other))
    }
}

#[cfg(test)]
mod append {
    use super::*;

    #[test]
    fn can_append_two_twotuples() {
        let left = ('a', ("b", ()));
        let right = (3, (true, ()));
        let result = left.append(right);
        assert_eq!(result, ('a', ("b", (3, (true, ())))));
    }

    #[test]
    fn appending_a_twotuple_to_an_empty_tuple_gives_the_two_tuple() {
        let left = ();
        let right = (3, ());
        let result = left.append(right);
        assert_eq!(result, (3, ()));
    }

    // #[test]
    // fn appending_a_bare_type_to_a_twotuple_places_the_type_as_head_on_a_twotuple_before_appending()
    // {
    //     let left = (3, ());
    //     let right = 'a';
    //     let result = left.append(right);
    //     assert_eq!(result, (3, ('a', ())));
    // }

    // fn appending_a_bare_type_to_an_empty_tuple_places_the_bare_type_as_head_of_an_otherwise_empty_twotuple(
    // ) {
    //     let left = ();
    //     let right = 'a';
    //     let result = left.append(right);
    //     assert_eq!(result, ('a', ()));
    // }
}

trait Split<A> {
    type Rest;

    fn split(self) -> (Option<A>, Self::Rest);
}

impl<Head, Tail> Split<Head> for (Head, Tail) {
    type Rest = Tail;

    fn split(self) -> (Option<Head>, Self::Rest) {
        (Some(self.0), self.1)
    }
}

impl<Head> Split<Head> for (Head,) {
    type Rest = ();

    fn split(self) -> (Option<Head>, Self::Rest) {
        (None, ())
    }
}

trait Next<A> {
    type NewSelf;

    fn next(self) -> A;
}
