create table users_songs (
    primary key (user_id, song_id),
    user_id int references users (user_id) on update cascade,
    song_id int references songs (song_id) on update cascade on delete cascade,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);