use disir_c;

pub fn groups (instance: &disir_c::Instance) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    // TODO: Query the instance for available groups

    // FIXME: Remove the hardcode 'test' group when query from instance is implemented
    vec.push (String::from("test"));

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_returns() {
        let mut instance : disir_c::Instance = Default::default();

        let g = groups(&instance);
        assert_eq!(g.len(), 1);
        assert_eq!(g[0], "test")
    }
}
