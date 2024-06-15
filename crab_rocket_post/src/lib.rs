pub mod controllers {
    pub mod post_controller;
}

pub mod mappers {
    pub mod post_mapper;
}

pub mod models {
    pub mod post;
}

pub mod routes {
    pub mod post_param;
    pub mod post_route;
}

pub mod services {
    pub mod post_service;
    pub mod r#impl {
        pub mod post_impl;
    }
}
