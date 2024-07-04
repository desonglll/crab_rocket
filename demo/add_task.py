import random
from faker import Faker
import requests

fake = Faker()


def send_post_request(url, data):
    try:
        response = requests.post(url, json=data)
        if response.status_code == 200:
            print("成功发送POST请求:", response.json())
        else:
            print("POST请求失败:", response.status_code)
    except Exception as e:
        print("发送POST请求时发生错误:", str(e))


def generate_random_data():
    title = fake.sentence(nb_words=random.randint(2, 6), variable_nb_words=True)
    body = fake.paragraph(nb_sentences=random.randint(2, 6), variable_nb_sentences=True)
    user_id = random.randint(1, 1)
    return {"title": title, "content": body, "user_id": user_id}


if __name__ == "__main__":
    url = "http://localhost:8000/api/task"  # 请将URL替换为实际的目标URL
    for _ in range(10000):
        random_data = generate_random_data()
        send_post_request(url, random_data)
