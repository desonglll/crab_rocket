use rocket::{Route, routes};

use crab_rocket_category::routes::category_route::*;
use crab_rocket_customer::routes::customer_route::*;
use crab_rocket_employee::routes::employee_route::*;
use crab_rocket_file::routes::{bin_file_route, form_file_route};
use crab_rocket_follow::routes::follow_route::*;
use crab_rocket_info::routes::info_route;
use crab_rocket_permission::routes::permission_route::*;
use crab_rocket_post::routes::post_route::*;
use crab_rocket_product::routes::product_route::*;
use crab_rocket_role::routes::role_route::*;
use crab_rocket_supplier::routes::supplier_route::*;
use crab_rocket_task::routes::task_route::*;
use crab_rocket_user::routes::user_route::*;

pub fn entities_routes() -> Vec<Route> {
    routes![
        bin_file_route::files,
        bin_file_route::retrieve_bin,
        bin_file_route::upload_bin,
        form_file_route::upload,
        form_file_route::upload_avatar,
        form_file_route::download_file,
        form_file_route::retrieve_file,
        form_file_route::get_all_files,
        form_file_route::file_stream,
        form_file_route::options_upload,
        info_route::get_info,
        // task routes
        get_tasks,
        filter_tasks,
        get_task_by_id,
        insert_single_task,
        delete_task_by_id,
        update_task_by_id,
        //user routes
        get_users,
        filter_users,
        get_user_by_id,
        insert_single_user,
        delete_user_by_id,
        update_user_by_id,
        options_user,
        // post routes
        get_posts,
        filter_posts,
        get_post_by_id,
        insert_single_post,
        delete_post_by_id,
        update_post_by_id,
        // follow routes
        get_follows,
        filter_follows,
        insert_single_follow,
        delete_follow_by_id,
        update_follow_by_id,
        delete_follow_specifically,
        //employee routes
        get_employees,
        filter_employees,
        get_employee_by_id,
        insert_single_employee,
        delete_employee_by_id,
        update_employee_by_id,
        options_employee,
        // role routes
        get_roles,
        filter_roles,
        get_role_by_id,
        insert_single_role,
        delete_role_by_id,
        update_role_by_id,
        options_role,
        // permission routes
        get_permissions,
        filter_permissions,
        get_permission_by_id,
        insert_single_permission,
        delete_permission_by_id,
        update_permission_by_id,
        options_permission,
        // supplier routes
        get_suppliers,
        filter_suppliers,
        get_supplier_by_id,
        insert_single_supplier,
        delete_supplier_by_id,
        update_supplier_by_id,
        options_supplier,
        //category routes
        get_categorys,
        filter_categorys,
        get_category_by_id,
        insert_single_category,
        delete_category_by_id,
        update_category_by_id,
        options_category,
        //product routes
        get_products,
        filter_products,
        get_product_by_id,
        insert_single_product,
        delete_product_by_id,
        update_product_by_id,
        options_product,
        //customer routes
        get_customers,
        filter_customers,
        get_customer_by_id,
        insert_single_customer,
        delete_customer_by_id,
        update_customer_by_id,
        options_customer,
    ]
}
