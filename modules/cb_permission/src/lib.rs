pub mod models {
    pub mod permission;
}

pub mod mappers {
    pub mod permission_mapper;
}

pub mod controllers {
    pub mod permission_controller;
}

pub mod routes {
    pub mod permission_route;
}
pub mod services {
    pub mod permission_service;
    pub mod r#impl {
        pub mod permission_impl;
    }
}
