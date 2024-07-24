use sea_orm::entity::prelude::*;

pub fn generate_vietname_now() -> DateTimeWithTimeZone {
    let vn_tz =
        chrono::FixedOffset::east_opt(7 * 3600).expect("Must be valid vietnam timezone offset");
    chrono::Utc::now().with_timezone(&vn_tz)
}
