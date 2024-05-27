// @generated automatically by Diesel CLI.

diesel::table! {
    department_table (department_id) {
        department_id -> Int4,
        #[max_length = 255]
        department_name -> Varchar,
        manager_id -> Nullable<Int4>,
        #[max_length = 255]
        location -> Nullable<Varchar>,
        creation_date -> Nullable<Timestamp>,
        last_update -> Nullable<Timestamp>,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        budget -> Nullable<Int4>,
        number_of_employees -> Nullable<Int4>,
        parent_department_id -> Nullable<Int4>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        state -> Nullable<Varchar>,
        #[max_length = 255]
        postal_code -> Nullable<Varchar>,
    }
}

diesel::table! {
    employee_table (employee_id) {
        employee_id -> Int4,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        employee_name -> Varchar,
        #[max_length = 255]
        gender -> Nullable<Varchar>,
        date_of_birth -> Nullable<Timestamp>,
        hire_date -> Nullable<Timestamp>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        phone_number -> Nullable<Varchar>,
        department_id -> Nullable<Int4>,
        #[max_length = 255]
        job_title -> Nullable<Varchar>,
        salary -> Nullable<Int4>,
        manager_id -> Nullable<Int4>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        state -> Nullable<Varchar>,
        #[max_length = 255]
        postal_code -> Nullable<Varchar>,
        valid -> Nullable<Bool>,
        last_update -> Nullable<Timestamp>,
        #[max_length = 255]
        role_name -> Nullable<Varchar>,
        role_id -> Nullable<Int4>,
    }
}

diesel::table! {
    follows (follow_id) {
        following_user_id -> Int4,
        followed_user_id -> Int4,
        created_at -> Nullable<Timestamp>,
        follow_id -> Int4,
    }
}

diesel::table! {
    posts (post_id) {
        post_id -> Int4,
        #[max_length = 255]
        title -> Nullable<Varchar>,
        body -> Nullable<Text>,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
    }
}

diesel::table! {
    role_table (role_id) {
        role_id -> Int4,
        #[max_length = 255]
        role_name -> Varchar,
        #[max_length = 255]
        description -> Nullable<Varchar>,
        #[max_length = 255]
        permissions -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    user_permission (id) {
        id -> Int4,
        role_id -> Int4,
        #[max_length = 255]
        permission_name -> Varchar,
        permission_description -> Nullable<Text>,
        #[max_length = 255]
        resource -> Varchar,
        #[max_length = 50]
        action -> Varchar,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        created_by -> Nullable<Varchar>,
        #[max_length = 255]
        updated_by -> Nullable<Varchar>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    user_table (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        role_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        fullname -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        mobile_phone -> Varchar,
    }
}

diesel::joinable!(tasks -> user_table (user_id));
diesel::joinable!(user_permission -> role_table (role_id));
diesel::joinable!(user_table -> role_table (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    department_table,
    employee_table,
    follows,
    posts,
    role_table,
    tasks,
    user_permission,
    user_table,
);
