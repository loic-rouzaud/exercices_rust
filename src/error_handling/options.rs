fn find_first<T, F>(items: &[T], predicate: F) -> Option<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().find(|item| predicate(item)) // find renvoie deja Some(..) ou None
}
