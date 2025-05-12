pub fn apply_operations<F>(data: &mut Vec<i32>, mut operations: Vec<F>)
where
    F: FnMut(&mut Vec<i32>),
{
    for operation in operations.iter_mut() {
        operation(data);
    }
}
