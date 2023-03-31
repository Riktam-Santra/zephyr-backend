# zephyr-backend

A backend for apps to connect different communities.

## Running the Application

1. Make sure you have a postgres local instance running.

2. set an environment variable using `export DB_CONFIG="host=localhost dbname=[DATABASE_NAME] user=[POSTGRES_USERNAME] password=[POSTGRES_USER_PASSWORD]."` replacing all the placeholders with your own adjustments.

3. clone this repository using `git clone https://github.com/Riktam-Santra/zephyr-backend.git`

4. cd into the directory using `cd zephyr-backend`

5. run the app using `cargo run`.

## Endpoints

### Create a user `POST /users`
body must be a json containing the username and the password.
```json
{
  "username": "some_username",
  "password": "some_password"
}
```
### Login a user `POST /users/auth/login`
body must be a json containing the username and the password.
```json
{
  "username": "some_username",
  "password": "some_password"
}
```
The response is a JWT token.

### Getting all users `GET /users`
A list of all users is returned.

### Posting a question `POST /questions`
body must be a json containing the title and the subtitle.
```json
{
  "title": "some_title",
  "subtitle": "some_subtitle"
}
```
**Must include bearer auth token for user**

### Get all questions `GET /questions`
Returns a list of all questions.
