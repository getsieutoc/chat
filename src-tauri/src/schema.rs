// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Nullable<Text>,
        name -> Text,
        model_name -> Text,
        parameters_config -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    conversations (id) {
        id -> Nullable<Integer>,
        agent_id -> Text,
        title -> Nullable<Text>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        status -> Nullable<Text>,
    }
}

diesel::table! {
    documents (id) {
        id -> Nullable<Text>,
        title -> Text,
        path -> Nullable<Text>,
        metadata -> Nullable<Text>,
        imported_at -> Nullable<Timestamp>,
        digested_at -> Nullable<Timestamp>,
        digest_status -> Nullable<Text>,
        digest_error -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(conversations -> agents (agent_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    conversations,
    documents,
);
