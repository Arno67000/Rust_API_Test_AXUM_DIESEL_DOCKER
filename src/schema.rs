// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    teams (id) {
        id -> Int8,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        nickname -> Varchar,
        level -> Int8,
        score -> Int8,
        #[max_length = 255]
        team_name -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    teams,
    users,
);
