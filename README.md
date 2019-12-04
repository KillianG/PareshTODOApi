# API Documentation

## Open Endpoints

Open endpoints require no Authentication.

* [Login](doc/login.md) : `POST /user/login`
* [Register](doc/register.md) : `POST /user/register`
* [Refresh Token](doc/refresh_token.md) : `POST /user/refresh_token`
* [Create team](doc/create_team.md) : `POST /team/create`
* [Join team](doc/invite_team.md) : `POST /team/invite`
* [My teams](doc/my_teams.md) : `GET /team/my`
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

### Current User related

Each endpoint manipulates or displays information related to the User whose
Token is provided with the request:

Nothing yet

### Account related

Endpoints for viewing and manipulating the Accounts that the Authenticated User
has permissions to access.

Nothing yet
