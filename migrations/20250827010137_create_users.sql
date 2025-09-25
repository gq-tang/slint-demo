-- Add migration script here
create table
    users (
        id integer primary key AUTOINCREMENT,
        username text not null,
        age integer not null,
        gender text not null,
        email text,
        created_at timestamp default CURRENT_TIMESTAMP,
        updated_at timestamp
    );