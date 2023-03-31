use actix_web::web::Query;

use crate::{
    enums::order_enums::OrderEnums,
    models::filter_models::{question_filters::QuestionFilters, user_filters::UserSearchFilters},
};

pub fn parse_question_search_filters(filters: Query<QuestionFilters>) -> String {
    let mut query = String::new();

    if let Some(order_by) = filters.order_by {
        if order_by == OrderEnums::LatestUploaded {
            query.push_str(" ORDER BY created_at DESC ");
        } else if order_by == OrderEnums::OldestUploaded {
            query.push_str(" ORDER BY created_at ASC ");
        }
    }

    if let Some(ques_uuid) = &filters.ques_uuids {
        let mut val: String = String::from("");
        for val_str in ques_uuid.into_iter() {
            val.push_str(format!("${}, ", val_str).as_str());
        }
        query.push_str(format!(" WHERE uuid in ({})", val).as_str());
    }

    if let Some(title) = &filters.title {
        if filters.ques_uuids.is_none() {
            query.push_str(&format!(" WHERE title like '%{}%' ", title));
        } else {
            query.push_str(&format!(" AND title like '%{}%' ", title));
        }
    }

    if let Some(v) = filters.answered {
        if v {
            if filters.title.is_none() && filters.ques_uuids.is_none() {
                query.push_str(" WHERE answer_id IS NOT NULL ");
            } else {
                query.push_str(" AND answer_id IS NOT NULL ");
            }
        } else {
            if filters.title.is_none() && filters.ques_uuids.is_none() {
                query.push_str(" WHERE answer_id IS NULL ");
            } else {
                query.push_str(" AND answer_id IS NULL ");
            }
        }
    };

    if let Some(offset) = filters.offset {
        query.push_str(&format!(" OFFSET {} ", offset));
    }

    match filters.limit {
        Some(limit) => {
            if limit < 10 {
                query.push_str(&format!(" LIMIT {} ", limit));
            } else {
                query.push_str(" LIMIT 10 ");
            }
        }
        None => {
            query.push_str(" LIMIT 10 ");
        }
    }
    return query;
}

pub fn parse_user_search_filters(filters: Query<UserSearchFilters>) -> String {
    let mut query = String::new();

    if let Some(x) = &filters.uuids {
        let mut sub_query = String::new();
        for (i, _) in x.into_iter().enumerate() {
            sub_query.push_str(format!("${}", i).as_str());
        }
        query.push_str(&sub_query);
    }
    query
}
