// @generated automatically by Diesel CLI.

diesel::table! {
    category_table (category_id) {
        category_id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        parent_id -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

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
    file_table (file_id) {
        file_id -> Uuid,
        file_name -> Varchar,
        file_url -> Varchar,
        uploaded_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    follow_table (follow_id) {
        following_user_id -> Int4,
        followed_user_id -> Int4,
        created_at -> Nullable<Timestamp>,
        follow_id -> Int4,
    }
}

diesel::table! {
    permission_table (permission_id) {
        permission_id -> Int4,
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
    post_table (post_id) {
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
    product_table (product_id) {
        product_id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 50]
        sku -> Varchar,
        #[max_length = 255]
        image -> Nullable<Varchar>,
        price -> Nullable<Numeric>,
        discount_price -> Nullable<Numeric>,
        is_discounted -> Nullable<Bool>,
        is_valid -> Nullable<Bool>,
        stock_quantity -> Nullable<Int4>,
        is_in_stock -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        supplier_id -> Nullable<Int4>,
        weight -> Nullable<Numeric>,
        #[max_length = 50]
        dimensions -> Nullable<Varchar>,
        #[max_length = 20]
        status -> Nullable<Varchar>,
        public -> Nullable<Bool>,
    }
}

diesel::table! {
    reload_counts (reload_date) {
        reload_date -> Date,
        count -> Int4,
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
    supplier_table (supplier_id) {
        supplier_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    task_table (task_id) {
        task_id -> Int4,
        title -> Text,
        content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Nullable<Int4>,
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
        full_name -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_url -> Nullable<Varchar>,
        bio -> Nullable<Text>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        mobile_phone -> Varchar,
    }
}

diesel::joinable!(product_table -> supplier_table (supplier_id));
diesel::joinable!(product_table -> user_table (user_id));
diesel::joinable!(task_table -> user_table (user_id));
diesel::joinable!(user_table -> role_table (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    category_table,
    department_table,
    employee_table,
    file_table,
    follow_table,
    permission_table,
    post_table,
    product_table,
    reload_counts,
    role_table,
    supplier_table,
    task_table,
    user_table,
);
