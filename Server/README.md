# Server code

This folder contains the code used to run the server for storing and accessing user and session data.

## Usage

To start the server, run the following

```shell
cargo run --release
```

## API use

to use the API, you need the API key which is stored in [`config.toml`](config.toml). This needs to be set in a cookie
with the key `key`.

### Submit data

To submit data, you will need to make a POST request to `/api/submit`. The request body will be the data to submit and
must have the general form as follows:

```json
{
  "user_id": "user ID",
  "start_time": 1671080669,
  "hr_data": [
    {
      "time": 1671080743,
      "pulse": 50
    }
  ]
}
```
Data that does not have this format will result ina 400 status code being returned (bad request).
