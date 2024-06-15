pub mod models {
    pub mod follow;
}

pub mod mappers {
    pub mod follow_mapper;
}

pub mod controllers {
    pub mod follow_controller;
}

pub mod routes {
    pub mod follow_param;
    pub mod follow_route;
}
pub mod services {
    pub mod follow_service;
    pub mod r#impl {
        pub mod follow_impl;
    }
}
