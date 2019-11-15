# Login

Used to collect a Token for a registered User.

**URL** : `/user/refresh_token`

**Method** : `POST`

**Auth required** : NO

**Data constraints**

```json
{
    "refresh_token": "[valid refresh token]"
}
```

**Data example**

```json
{
    "refresh_token": "ZPOzpoadZPDoZdpqaozdZDP"
}
```

## Success Response

**Code** : `OK`

**Content example**

```json
{
    "access_token": "azidohdzaodh2oaiea2312oI3aze",
    "refresh_token": "OIDodiazjdpoIJDOIJDz"
}
```

