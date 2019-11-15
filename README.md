# API Documentation

## Open Endpoints

Open endpoints require no Authentication.

* [Login](doc/login.md) : `POST /user/login`
* [Register](doc/register.md) : `POST /user/register`


## Endpoints that require Authentication

Closed endpoints require a valid Token to be included in the header of the
request. A Token can be acquired from the Login view above.

### Current User related

Each endpoint manipulates or displays information related to the User whose
Token is provided with the request:

Nothing yet

### Account related

Endpoints for viewing and manipulating the Accounts that the Authenticated User
has permissions to access.

Nothing yet
