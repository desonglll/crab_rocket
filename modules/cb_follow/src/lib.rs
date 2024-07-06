pub mod models {
    pub mod follow;
    pub mod follow_filter;
}

pub mod mappers {
    pub mod follow_mapper;
    pub mod follow_mapper_trait;
}

pub mod controllers {
    pub mod follow_controller;
    pub mod follow_controller_trait;
}

pub mod routes {
    pub mod follow_param;
    pub mod follow_route;
}
pub mod services {
    pub mod follow_service;
    pub mod follow_service_trait;
}
