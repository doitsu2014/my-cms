pub mod create;
pub mod read;

#[cfg(test)]
pub mod tests {
    use fake::{faker::lorem::en::Word, Fake};
    use rand::{rngs::StdRng, SeedableRng};

    pub fn fake_tag_names(number_of_tags: usize) -> Vec<String> {
        let seed = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let r = &mut StdRng::from_seed(seed);
        let tag_names: Vec<String> = (0..number_of_tags)
            .map(|_| Word().fake_with_rng(r))
            .collect();

        tag_names
    }
}
