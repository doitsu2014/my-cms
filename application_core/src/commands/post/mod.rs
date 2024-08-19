pub mod create;
pub mod delete;
pub mod modify;
pub mod read;

#[cfg(test)]
pub mod test {
    use fake::{
        faker::lorem::en::{Paragraphs, Words},
        Fake, Faker,
    };
    use uuid::Uuid;

    use crate::commands::tag::tests::fake_tag_names;

    use super::create::create_request::CreatePostRequest;

    pub fn fake_create_post_request(category_id: Uuid, number_of_tags: usize) -> CreatePostRequest {
        let title_words: Vec<String> = Words(5..10).fake();
        let content_paragraphs: Vec<String> = Paragraphs(2..5).fake();
        let filenames: Vec<String> = vec![
            "image1.jpg".to_string(),
            "image2.jpg".to_string(),
            "image3.jpg".to_string(),
        ];
        CreatePostRequest {
            title: title_words.join(" "),
            preview_content: Some(content_paragraphs[0].clone()),
            content: content_paragraphs.join("\n"),
            published: Faker.fake::<bool>(),
            tag_names: Some(fake_tag_names(number_of_tags)),
            category_id,
            thumbnail_paths: filenames.to_owned(),
        }
    }
}
