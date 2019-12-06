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

* [Create team](doc/create_team.md) : `POST /team/create`
* [Join team](doc/invite_team.md) : `POST /team/invite`
* [My teams](doc/my_teams.md) : `GET /team/my`
* [Set location](doc/set_location.md) : `POST /user/location/<country_code>`
* [Get location](doc/get_location.md) : `GET /user/location`
* [Exist](doc/exist.md) : `GET /user/exist`
* [Members](doc/members.md) : `GET /team/members/<team_name>`
* [Profile picture](doc/picture.md) : `POST /user/picture`
* [Leave team](doc/leave.md) : `POST /team/leave/<team_name>`



