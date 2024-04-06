FORMAT: 1A
HOST: your-api-host.com

# Task Management API

This API allows you to manage tasks.

- [Task Management API](#task-management-api)
  - [Tasks Collection \[/task\]](#tasks-collection-task)
    - [Get All Tasks \[GET\]](#get-all-tasks-get)
    - [Create a Task \[POST\]](#create-a-task-post)
  - [Single Task \[/task/{id}\]](#single-task-taskid)
    - [Get a Task \[GET\]](#get-a-task-get)
    - [Update a Task \[PATCH\]](#update-a-task-patch)
    - [Replace a Task \[PUT\]](#replace-a-task-put)
    - [Delete a Task \[DELETE\]](#delete-a-task-delete)
  - [Miscellaneous](#miscellaneous)
    - [Hello World \[/\]](#hello-world-)
    - [Demo Endpoint \[/test\]](#demo-endpoint-test)
  - [Users Collection \[/user\]](#users-collection-user)
    - [Get All Users \[GET\]](#get-all-users-get)
    - [Create a User \[POST\]](#create-a-user-post)
  - [Single User \[/user/{id}\]](#single-user-userid)
    - [Get a User \[GET\]](#get-a-user-get)
    - [Update a User \[PATCH\]](#update-a-user-patch)
    - [Delete a User \[DELETE\]](#delete-a-user-delete)

## Tasks Collection [/task]

### Get All Tasks [GET]

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": [
      {
        "id": 1,
        "title": "Task 1",
        "content": "Task 1 content",
        "created_at": "2024-04-06T12:00:00Z",
        "updated_at": "2024-04-06T12:00:00Z"
      },
      {
        "id": 2,
        "title": "Task 2",
        "content": "Task 2 content",
        "created_at": "2024-04-06T12:00:00Z",
        "updated_at": "2024-04-06T12:00:00Z"
      }
    ]
  }
  ```

### Create a Task [POST]

- Request (application/json)

  ```json
  {
    "title": "New Task",
    "content": "New Task content"
  }
  ```

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 3,
      "title": "New Task",
      "content": "New Task content",
      "created_at": "2024-04-06T12:00:00Z",
      "updated_at": "2024-04-06T12:00:00Z"
    }
  }
  ```

## Single Task [/task/{id}]

- Parameters
  - id (integer) - ID of the Task

### Get a Task [GET]

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 1,
      "title": "Task 1",
      "content": "Task 1 content",
      "created_at": "2024-04-06T12:00:00Z",
      "updated_at": "2024-04-06T12:00:00Z"
    }
  }
  ```

### Update a Task [PATCH]

- Request (application/json)

  ```json
  {
    "title": "Updated Task 1",
    "content": "Updated Task 1 content",
    "updated_at": "2024-04-06T12:00:00Z"
  }
  ```

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 1,
      "title": "Updated Task 1",
      "content": "Updated Task 1 content",
      "created_at": "2024-04-06T12:00:00Z",
      "updated_at": "2024-04-06T12:00:00Z"
    }
  }
  ```

### Replace a Task [PUT]

- Request (application/json)

  ```json
  {
    "title": "Replaced Task 1",
    "content": "Replaced Task 1 content",
    "updated_at": "2024-04-06T12:00:00Z"
  }
  ```

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 1,
      "title": "Replaced Task 1",
      "content": "Replaced Task 1 content",
      "created_at": "2024-04-06T12:00:00Z",
      "updated_at": "2024-04-06T12:00:00Z"
    }
  }
  ```

### Delete a Task [DELETE]

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 1,
      "title": "Replaced Task 1",
      "content": "Replaced Task 1 content",
      "created_at": "2024-04-06T12:00:00Z",
      "updated_at": "2024-04-06T12:00:00Z"
    }
  }
  ```

## Miscellaneous

### Hello World [/]

### Demo Endpoint [/test]

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "user": {
        "id": 123,
        "name": "John Doe",
        "email": "john@example.com"
      }
    }
  }
  ```

## Users Collection [/user]

### Get All Users [GET]

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": [
      {
        "id": 1,
        "name": "User 1",
        "email": "user1@example.com"
      },
      {
        "id": 2,
        "name": "User 2",
        "email": "user2@example.com"
      }
    ]
  }
  ```

### Create a User [POST]

- Request (application/json)

  ```json
  {
    "name": "New User",
    "email": "newuser@example.com"
  }
  ```

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 3,
      "name": "New User",
      "email": "newuser@example.com"
    }
  }
  ```

## Single User [/user/{id}]

- Parameters
  - id (integer) - ID of the User

### Get a User [GET]

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 1,
      "name": "User 1",
      "email": "user1@example.com"
    }
  }
  ```

### Update a User [PATCH]

- Request (application/json)

  ```json
  {
    "name": "Updated User",
    "email": "updateduser@example.com"
  }
  ```

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 1,
      "name": "Updated User",
      "email": "updateduser@example.com"
    }
  }
  ```

### Delete a User [DELETE]

- Response 200 (application/json)

  ```json
  {
    "code": 200,
    "message": "success",
    "data": {
      "id": 1,
      "name": "Updated User",
      "email": "updateduser@example.com"
    }
  }
  ```
