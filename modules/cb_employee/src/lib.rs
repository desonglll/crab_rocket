pub mod models {
    pub mod employee;
}

pub mod mappers {
    pub mod employee_mapper;
}

pub mod controllers {
    pub mod employee_controller;
}

pub mod routes {
    pub mod employee_param;
    pub mod employee_route;
}
pub mod services {
    pub mod employee_service;
    pub mod r#impl {
        pub mod employee_impl;
    }
}
