create table songs (
    song_id serial not null primary key,
    title text not null,
    file_url text not null, 
    track_length text not null,
    image_url text,
    artist_id integer not null references users(user_id),
    album_id integer references albums(album_id),
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);