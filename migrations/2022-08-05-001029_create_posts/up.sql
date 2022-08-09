-- Your SQL goes herecreat
create table post (
    id serial primary key,
    title varchar not null,
    slug varchar not null,
    body TEXT not null
)