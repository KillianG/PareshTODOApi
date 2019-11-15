# Login

Used to create a new account

**URL** : `/user/register`

**Method** : `POST`

**Auth required** : NO

**Data constraints**

```json
{
    "username": "[valid email address]",
    "password": "[password in plain text]"
}
```

**Data example**

```json
{
    "username": "iloveauth@example.com",
    "password": "abcd1234"
}
```

## Success Response

**Code** : `CREATED`

## Error Response

**Condition** : If one field is missing

**Code** : `BAD REQUEST`

___

**Condition** : If user already exist

**Code** : `CONFLICT`

___

**Condition** : If JSON is misformated

**Code** : `UNPROCESSABLE ENTITY`
