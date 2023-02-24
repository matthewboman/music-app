create table albums (
    album_id serial not null primary key,
    title text not null,
    image_url text,
    details text, 
    artist_id integer not null references users(user_id),
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);