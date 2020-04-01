# API Documentation

## Open Endpoints

Open endpoints require no Authentication.

* [Login](doc/login.md) : `POST /user/login`
* [Register](doc/register.md) : `POST /user/register`
* [Refresh Token](doc/refresh_token.md) : `POST /user/refresh_token`

---
## Endpoints that require Authentication

Closed endpoints require a valid Token to be included in the header of the
request. A Token can be acquired from the Login view above.

```json
{
    "header": {
        "Authorization": "[valid token]"
        }
}
```

* [Exist](doc/exist.md) : `GET /user/exist`



