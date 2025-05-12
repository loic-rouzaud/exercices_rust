use std::rc::Rc;

pub fn create_shared_resource() -> Rc<Vec<i32>> {
    let my_vec = vec![1, 2, 3, 4, 5];
    let shared_data = Rc::new(my_vec);
    shared_data
}

pub fn add_consumer(resource: &Rc<Vec<i32>>) -> usize {
    let _consumer = Rc::clone(resource);
    Rc::strong_count(resource)
}

#[test]
fn test_shared_resource() {
    let original_vec = create_shared_resource();
    assert_eq!(Rc::strong_count(&original_vec), 1);

    let count1 = add_consumer(&original_vec);
    assert_eq!(count1, 3);

    let count2 = add_consumer(&original_vec);
    assert_eq!(count2, 3);
}
