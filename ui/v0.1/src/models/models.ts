export interface SubmittedRole {
    role_name: string,
    permissions: string,
    created_at: string,
    updated_at: string,
    description: string
}


export interface Permission {
    id: number,
    permission_name: string,
    permission_description: string | null,
    resource: string,
    action: string,
    is_active: boolean | null,
    created_at: string | null,
    updated_at: string | null,
    created_by: string | null,
    updated_by: string | null,
    notes: string | null,
}

export interface User {
    user_id: number,
    username: string,
    role_id: number | null,
    created_at: string | null,
    email: string | null,
    password: string,
    fullname: string | null,
    avatar_url: string | null,
    bio: string | null,
    updated_at: string | null,
    mobile_phone: string,
}

export interface PatchUser {
    username: string,
    role_id: string | null,
    created_at: string | null,
    email: string | null,
    password: string,
    fullname: string | null,
    avatar_url: string | null,
    bio: string | null,
    updated_at: string | null,
    mobile_phone: string,
}

export interface Employee {
    employee_id: number;
    first_name?: string | null;
    last_name?: string | null;
    employee_name: string;
    gender?: string | null;
    date_of_birth?: string | null; // Assuming it's a string representation of datetime
    hire_date?: string | null; // Assuming it's a string representation of datetime
    email?: string | null;
    phone_number?: string | null;
    department_id?: number | null;
    job_title?: string | null;
    salary?: number | null;
    manager_id?: number | null;
    address?: string | null;
    city?: string | null;
    state?: string | null;
    postal_code?: string | null;
    valid?: boolean | null;
    last_update?: string | null; // Assuming it's a string representation of datetime
}

export interface Post {
    post_id: number;
    title: string;
    body: string;
    user_id: number;
    status: string;
    created_at: string;
    updated_at: string;
    username: string;
}

export interface PatchPost {
    title: string;
    body: string;
    user_id: number;
    status: string;
    created_at: string;
    updated_at: string;
}

export interface Task {
    id: number;
    title: string;
    content: string;
    user_id: number;
    created_at: string;
    updated_at: string;
}

export interface PatchTask {
    content: string;
    title: string;
    user_id: number;
}

export interface File {
    id: string,
    file_name: string,
    file_url: string,
    uploaded_at: string
}