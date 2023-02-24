create table tags (
    tag_id serial not null primary key,
    name text not null,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);