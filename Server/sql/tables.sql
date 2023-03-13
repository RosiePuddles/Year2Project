CREATE TABLE users (
    uname   varchar(30) unique,
    uuid    serial primary key
);
CREATE TABLE sessions (
    uuid        int REFERENCES users(uuid),
    time_start  timestamp with time zone,
    hr          int[],
    meditation  int[],
    gaze        point[]
);
CREATE TABLE keys (
    key         char(512) unique,
    uuid        int REFERENCES users(uuid),
    end_time    timestamp with time zone
);
CREATE TABLE admins (
    email   text unique,
    pwdh    char(128),
    uuid    serial primary key
);
CREATE TABLE admin_auth (
    auth_key    char(512) unique,
    uuid        int REFERENCES admins(uuid),
    end_time    timestamp with time zone
);
CREATE TABLE admin_blocked (
    email   text unique
);
