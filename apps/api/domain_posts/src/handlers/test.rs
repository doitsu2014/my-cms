//! Shared test fixtures for the post domain's category handlers.
//!
//! Originally sourced from `application_core::commands::category::test` and
//! `application_core::commands::tag::tests` per the
//! `consolidate-category-ai-translate-into-domain-posts` change. Both legacy
//! modules have since been deleted; the canonical fixtures now live only in
//! this file (verified by the
//! `split-media-and-user-domains-merge-tags-into-posts` change). The fixtures
//! are `pub(crate)` so the surrounding `#[cfg(test)]` modules in
//! `handlers/category/**` can call them without leaking out of the crate.

use fake::{
    faker::lorem::en::{Word, Words},
    Fake,
};
use rand::{rngs::StdRng, SeedableRng};
use uuid::Uuid;

use crate::entities::sea_orm_active_enums::CategoryType;
use crate::handlers::category::create::create_request::CreateCategoryRequest;

pub(crate) fn fake_tag_names(number_of_tags: usize) -> Vec<String> {
    let seed = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];
    let r = &mut StdRng::from_seed(seed);
    let tag_names: Vec<String> = (0..number_of_tags)
        .map(|_| Word().fake_with_rng(r))
        .collect();

    tag_names
}

pub(crate) fn fake_create_category_request(number_of_tags: usize) -> CreateCategoryRequest {
    let words: Vec<String> = Words(2..5).fake();
    let display_name = words.join(" ");
    CreateCategoryRequest {
        display_name,
        category_type: CategoryType::Blog,
        tag_names: Some(fake_tag_names(number_of_tags)),
        parent_id: None,
        translations: None,
    }
}

pub(crate) fn fake_create_category_request_with_category_type(
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
        translations: None,
    }
}

pub(crate) fn fake_create_category_request_as_child(
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
        translations: None,
    }
}
