fn main() {
    // or
    let x = Some(2);
    let y = None;
    assert_eq!(x.or(y), Some(2));

    let x = None;
    let y = Some(100);
    assert_eq!(x.or(y), Some(100));

    let x = Some(2);
    let y = Some(100);
    assert_eq!(x.or(y), Some(2));

    let x: Option<u32> = None;
    let y = None;
    assert_eq!(x.or(y), None);

    // or_else
    fn nobody() -> Option<&'static str> {
        None
    }
    fn vikings() -> Option<&'static str> {
        Some("vikings")
    }

    assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
    assert_eq!(None.or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(nobody), None);

    // get_or_insert
    let mut x = None;

    {
        let y: &mut u32 = x.get_or_insert(5);
        assert_eq!(y, &5);

        *y = 7;
    }

    assert_eq!(x, Some(7));

    // get_or_insert_with
    let mut x = None;

    {
        let y: &mut u32 = x.get_or_insert_with(|| 5);
        assert_eq!(y, &5);

        *y = 7;
    }

    assert_eq!(x, Some(7));

    // get_or_insert_default
    let mut x = None;

    {
        let y: &mut u32 = x.get_or_insert_default();
        assert_eq!(y, &0);

        *y = 7;
    }

    assert_eq!(x, Some(7));
}
