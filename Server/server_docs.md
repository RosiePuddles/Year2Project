# Server Documentation

This file is the documentation for the server. This will include possible requests, required data and data formats, and response codes

## API

The API is intended top be used by the Oculus headset for logging-in, making new user accounts, and storing and accessing session data by users.

> **All API requests require the API key cookie. This cookie has the key value `key` and the value must match the value defined in [`config.toml`](config.toml). The incorrect value or not having set the cookie will result in a forbidden response (403)**

### Submit session data

This path is used to submit session data to the server. This path requires the session data to be in JSON form in the request body and have the following general form
```json
{
  "user_id": "user ID",
  "start_time": 1671080669,
  "hr_data": [
    {
      "time": 1671080743,
      "pulse": 50
    }
  ],
  "gaze_data": [
    {
      "time": 1671080743,
      "yaw": 50,
      "pitch": -3
    }
  ]
}
```

<table>
<tr><td>Request type</td><td>POST</td></tr>
<tr><td>Path</td><td><code>/api/submit</code></td></tr>
</table>

| Response (Code)             | Description                                                                  |
|-----------------------------|------------------------------------------------------------------------------|
| Ok (200)                    | Session data correctly saved                                                 |
| Bad request (400)           | Request body does not match required session data format (see example above) |
| Forbidden (403)             | Missing or incorrect API key                                                 |
| Gone (410)                  | User ID has been removed or never existed                                    |
| Internal server error (500) | Server unable to save session data                                           |

### New user

This path is used to create a new user account. This path requires new user data to be in JSON form in the request body and have the following general form
```json
{
  "uname": "username",
  "pin": "0000"
}
```

If this path returns Ok(200), the response body will be the new user ID

<table>
<tr><td>Request type</td><td>POST</td></tr>
<tr><td>Path</td><td><code>/api/new</code></td></tr>
</table>

| Response (Code)             | Description                                                          |
|-----------------------------|----------------------------------------------------------------------|
| Ok (200)                    | User added to database                                               |
| Bad request (400)           | Request body does not match required data format (see example above) |
| Forbidden (403)             | Missing or incorrect API key                                         |
| Conflict (409)              | Username already exists                                              |
| Internal server error (500) | Server unable to read users or unable to make new user               |

### Login

This path is used to log in a user. This path requires user data to be in JSON form in the request body and have the following general form
```json
{
  "uname": "username",
  "pin": "0000"
}
```

If this path returns Ok(200), the response body will be the user ID

<table>
<tr><td>Request type</td><td>POST</td></tr>
<tr><td>Path</td><td><code>/api/login</code></td></tr>
</table>

| Response (Code)             | Description                                                          |
|-----------------------------|----------------------------------------------------------------------|
| Ok (200)                    | User found and UID returned                                          |
| Bad request (400)           | Request body does not match required data format (see example above) |
| Unauthorised (401)          | Incorrect pin                                                        |
| Forbidden (403)             | Missing or incorrect API key                                         |
| Gone (410)                  | Username has been removed or never existed                           |
| Internal server error (500) | Server unable to read users                                          |

