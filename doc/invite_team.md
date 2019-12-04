# Join team

Used to invite a user to a team

**URL** : `/team/invite`

**Method** : `POST`

**Auth required** : YES

**Data constraints**

```json
{
    "team_name": "[valid team name]",
    "invited_username": "[valid username]"
}
```

**Data example**

```json
{
    "name": "BestTeam",
    "logo": "Killian"
}
```

## Success Response

**Code** : `OK`

## Error Response

**Condition** : If one field is missing

**Code** : `BAD REQUEST`

___

**Condition** : If JSON is misformated

**Code** : `UNPROCESSABLE ENTITY`
