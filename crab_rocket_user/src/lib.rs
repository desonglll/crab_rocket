pub mod models {
    pub mod user;
}

pub mod mappers {
    pub mod user_mapper;
}

pub mod controllers {
    pub mod user_controller;
}

pub mod routes {
    pub mod user_route;
}
pub mod services {
    pub mod user_service;
    pub mod r#impl {
        pub mod user_impl;
    }
}
