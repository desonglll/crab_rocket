import random
import string
from faker import Faker
import requests

fake = Faker()


def generate_username():
    return fake.user_name()


def generate_mobile_phone():
    return fake.phone_number()


def generate_password():
    # Generate a random password
    return "".join(random.choices(string.ascii_letters + string.digits, k=10))


def generate_new_user():
    return {
        "username": generate_username(),
        "role": None,  # You can modify this if needed
        "created_at": None,  # You can modify this if needed
        "email": None,  # You can modify this if needed
        "password": generate_password(),
        "fullname": None,  # You can modify this if needed
        "avatar_url": None,  # You can modify this if needed
        "bio": None,  # You can modify this if needed
        "updated_at": None,  # You can modify this if needed
        "mobile_phone": generate_mobile_phone(),
    }


def send_post_request(url, data):
    try:
        response = requests.post(url, json=data)
        if response.status_code == 200:
            print("成功发送POST请求:", response.json())
        else:
            print("POST请求失败:", response.status_code)
    except Exception as e:
        print("发送POST请求时发生错误:", str(e))


if __name__ == "__main__":
    url = "http://localhost:8000/user"  # 请将URL替换为实际的目标URL
    for _ in range(10000):
        new_user = generate_new_user()
        send_post_request(url, new_user)
