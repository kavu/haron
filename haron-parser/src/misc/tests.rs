use super::Core;

#[test]
fn test_core() {
    let mut core: Core<u64> = Core::with_capacity(4);

    assert_eq!(core.len(), 4);
    assert_eq!(core.capacity(), 4);
    assert_eq!(core.is_empty(), false);
    assert_eq!(core[3], None);

    core[0] = Some(0);
    core[1] = Some(1);
    core[3] = Some(3);

    assert_eq!(core.next(), Some(0));
    assert_eq!(core.next(), Some(1));
    assert_eq!(core.next(), None);
    assert_eq!(core.next(), Some(3));
    assert_eq!(core.next(), Some(0));
}