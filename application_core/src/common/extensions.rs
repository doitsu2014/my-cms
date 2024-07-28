pub trait StringExtension {
    fn to_slug(&self) -> String;
}

impl StringExtension for String {
    fn to_slug(&self) -> String {
        self.to_lowercase().replace(" ", "-")
    }
}
