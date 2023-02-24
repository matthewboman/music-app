create table album_tags (
    primary key (album_id, tag_id),
    album_id int references albums (album_id) on update cascade,
    tag_id int references tags (tag_id) on update cascade on delete cascade,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);