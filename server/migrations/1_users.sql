create table users (
    user_id serial not null primary key,
    email text not null,
    username text not null,
    image_url text,
    password text,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);

create unique index users__email__unique on users (email);
