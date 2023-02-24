create table artist_tags (
    primary key (artist_id, tag_id),
    artist_id int references artists (artist_id) on update cascade,
    tag_id int references tags (tag_id) on update cascade on delete cascade,
    created_at timestamp(3) default current_timestamp not null,
    updated_at timestamp(3) default current_timestamp not null
);