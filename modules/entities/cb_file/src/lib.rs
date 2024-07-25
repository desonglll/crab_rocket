pub mod models {
    pub mod file;
    pub mod file_response;
    pub mod upload;
}

pub mod mappers {
    pub mod file_mapper;
}

pub mod controllers {
    pub mod file_controller;
}

pub mod routes {
    pub mod bin_file_route;
    pub mod form_file_route;
}

pub mod services {
    pub mod file_service;
    pub mod file_service_trait;
    // pub mod image_service;
}
