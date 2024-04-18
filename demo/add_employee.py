import random
import string
from faker import Faker
import requests
from datetime import datetime

fake = Faker()


def generate_employee_name():
    return fake.name()


def generate_date_of_birth():
    return fake.date_of_birth(minimum_age=18, maximum_age=65).isoformat()


def generate_hire_date():
    return fake.date_time_between(start_date="-5y", end_date="now").isoformat()


def generate_email(employee_name):
    return f"{employee_name.lower().replace(' ', '_')}@example.com"


def generate_phone_number():
    return fake.phone_number()


def generate_department_id():
    # Replace this with logic to generate department IDs if needed
    return random.randint(1, 10)


def generate_job_title():
    return fake.job()


def generate_salary():
    return random.randint(30000, 100000)


def generate_address():
    return fake.address()


def generate_city():
    return fake.city()


def generate_state():
    return fake.state_abbr()


def generate_postal_code():
    return fake.postcode()


def generate_role_name():
    return fake.job()


def generate_new_employee():
    employee_name = generate_employee_name()
    return {
        "employee_name": employee_name,
        # "first_name": employee_name.split()[0],
        # "last_name": employee_name.split()[1],
        # "gender": random.choice(["Male", "Female"]),
        # "date_of_birth": generate_date_of_birth(),
        # "hire_date": generate_hire_date(),
        # "email": generate_email(employee_name),
        # "phone_number": generate_phone_number(),
        # "department_id": generate_department_id(),
        # "job_title": generate_job_title(),
        # "salary": generate_salary(),
        # "address": generate_address(),
        # "city": generate_city(),
        # "state": generate_state(),
        # "postal_code": generate_postal_code(),
        # "valid": True,
        # "role_name": generate_role_name(),
        # "role_id": random.randint(1, 5),
    }


def send_post_request(url, data):
    try:
        response = requests.post(url, json=data)
        if response.status_code == 200:
            print("Successfully sent POST request:", response.json())
        else:
            print("POST request failed:", response.status_code)
    except Exception as e:
        print("Error occurred while sending POST request:", str(e))


if __name__ == "__main__":
    url = "http://localhost:8000/employee"  # Replace with actual endpoint URL
    for _ in range(10000):
        new_employee = generate_new_employee()
        send_post_request(url, new_employee)
