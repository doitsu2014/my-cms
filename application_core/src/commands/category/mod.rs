pub mod create;
pub mod delete;
pub mod modify;
pub mod read;

#[cfg(test)]
pub mod test {
    use super::create::create_request::CreateCategoryRequest;
    use crate::{
        commands::tag::tests::fake_tag_names, entities::sea_orm_active_enums::CategoryType,
    };
    use fake::{faker::lorem::en::Words, Fake};
    use uuid::Uuid;

    pub fn fake_create_category_request(number_of_tags: usize) -> CreateCategoryRequest {
        let words: Vec<String> = Words(2..5).fake();
        let display_name = words.join(" ");
        CreateCategoryRequest {
            display_name,
            category_type: CategoryType::Blog,
            tag_names: Some(fake_tag_names(number_of_tags)),
            parent_id: None,
        }
    }

    pub fn fake_create_category_request_with_category_type(
        number_of_tags: usize,
        category_type: CategoryType,
    ) -> CreateCategoryRequest {
        let words: Vec<String> = Words(2..5).fake();
        let display_name = words.join(" ");
        CreateCategoryRequest {
            display_name,
            category_type: category_type.to_owned(),
            tag_names: Some(fake_tag_names(number_of_tags)),
            parent_id: None,
        }
    }

    pub fn fake_create_category_request_as_child(
        parent_id: Uuid,
        number_of_tags: usize,
    ) -> CreateCategoryRequest {
        let words: Vec<String> = Words(2..5).fake();
        let display_name = words.join(" ");
        CreateCategoryRequest {
            display_name,
            category_type: CategoryType::Blog,
            tag_names: Some(fake_tag_names(number_of_tags)),
            parent_id: Some(parent_id),
        }
    }
}
