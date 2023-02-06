# New changes

This file details the reasons for moving away from the first version of the server using Rocket and data in XML files, to using Actix and a Postgres database.

## Rocket to Actix

Rocket is mostly considered to be 'dead' now and is sitting undeveloped. As such it's recommended to use a different server framework.

Actix was chosen due to its similar "feel" to Rocket and good database support with useful examples. It's well-developed and has more than enough features for the intended application.

## XML to Postgres

Having writen a server that interfaces with XML files, it's a relatively tedious task. Each file can be very easily altered and thus requires validation each time it's opened; files may be moved or go missing so any attempt to access a file will require error handling, and sometimes error recovery. File writing can also result in errors which must be handled appropriately and recoveries must be attempted.

This is all a very tedious task that can be handled a lot better. The file structure intended was very similar to that of a database schema as well which makes using Postgres even more sensible.

For these reasons, along with having an example of how to use Actix and Postgres, we have opted to change to using a Postgres database. This means that the database will be easier to interface with, and require much less error handling leading to cleaner and nicer code. Because Postgres is a very large project, the read-write times are also likely to be greatly reduced. Postgres also allows for good scalability and simple backups without requiring large numbers of files be stored and accessed individually.

## Consequences

As a consequence of this change, the server has been rewritten. This has taken additional time but has allowed the code to be much cleaner and more readable. Due to Actix's similarity with Rocket and the requirement for less code to be written due to less error handling and recovery, the development time for this version was less that the time for the original server.
