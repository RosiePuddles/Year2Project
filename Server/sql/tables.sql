CREATE TABLE users (
    uname   varchar(30) unique,
    uuid    serial primary key
);
CREATE TABLE sessions (
    uuid        int REFERENCES users(uuid),
    time_start  timestamp with time zone,
    hr          int[],
    gaze        point[]
);
CREATE TABLE keys (
    key         char(512) unique,
    uuid        int REFERENCES users(uuid),
    end_time    timestamp with time zone
);
