pub mod models {
    pub mod file;
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
    pub mod r#impl {
        pub mod file_impl;
    }
}
