# My API

## Authentication [/auth]

### Login [POST]

- Request (application/json)

  - Attributes
    - username: john.doe (string, required) - The username of the user.
    - password: password123 (string, required) - The password of the user.

- Response 200 (application/json)
  - Attributes
    - token: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9 (string) - The authentication token.

## Users [/users]

### Get All Users [GET]

- Response 200 (application/json)
  - Attributes
    - users (array)
      - (object)
        - id: 1 (number) - The ID of the user.
        - username: john.doe (string) - The username of the user.
        - email: john.doe@example.com (string) - The email of the user.
