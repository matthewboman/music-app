create table users_albums (
    primary key (user_id, album_id),
    user_id int references users (user_id) on update cascade,
    album_id int references albums (album_id) on update cascade on delete cascade,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);