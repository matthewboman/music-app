create table artists (
    artist_id serial not null primary key,
    email text not null,
    username text not null,
    image_url text,
    bio text,
    password text not null,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);

create unique index artists__email__unique on artists (email);
