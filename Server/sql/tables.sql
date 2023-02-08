CREATE TABLE users (
    uname   varchar(30) unique,
    id      int primary key CHECK ( id >= 0 )
);
CREATE TABLE data (
    hr          int[],
    gaze        point[],
    time_start  timestamp with time zone,
    user_id     int REFERENCES users
);