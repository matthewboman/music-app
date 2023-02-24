INSERT INTO users (email, username, password)
VALUES ('fredy@cats.com', 'Fredy', 'meow');

INSERT INTO users (email, username, password)
VALUES ('maple@dogs.com', 'Maple', 'bark');

INSERT INTO artists (email, username, image_url, bio, password)
VALUES ('secretshameband@gmail.com', 'Secret Shame', 'https://f4.bcbits.com/img/0029683632_10.jpg', 'Dark post-punk', 'secret');

INSERT INTO artists (email, username, bio, password)
VALUES ('xor.music@protonmail.com', 'XOR', 'electronic music', 'secret');

INSERT INTO albums (title, image_url, artist_id)
VALUES ('Autonomy', 'https://f4.bcbits.com/img/a3248675637_10.jpg', 1);

INSERT INTO albums (title, details, artist_id)
VALUES ('Ephemeral EP', 'Recorded summer 2021', 2);

INSERT INTO songs (title, file_url, track_length, artist_id, album_id)
VALUES ('Pink Staircase', 'https://soundcloud.com/secretshame/secret-shame-pink-staircase', '1:46', 1, 1);

INSERT INTO songs (title, file_url, track_length, artist_id, album_id)
VALUES ('Zero', 'https://soundcloud.com/secretshame/secret-shame-zero', '5:05', 1, 1);

INSERT INTO songs (title, file_url, track_length, artist_id, album_id)
VALUES ('Persephone', 'https://soundcloud.com/secretshame/secret-shame-persephone', '5:52', 1, 1);

INSERT INTO songs (title, file_url, track_length, artist_id, album_id)
VALUES ('Coreopsis', 'https://xoravl.bandcamp.com/track/coreopsis', '6:13', 2, 2);

INSERT INTO songs (title, file_url, track_length, artist_id)
VALUES ('God Called in Sick Today', 'https://xoravl.bandcamp.com/track/god-called-in-sick-today', '4:54', 2);

INSERT INTO users_songs (user_id, song_id)
VALUES (1, 1);

INSERT INTO users_songs (user_id, song_id)
VALUES (1, 2);

INSERT INTO users_songs (user_id, song_id)
VALUES (1, 4);

INSERT INTO users_songs (user_id, song_id)
VALUES (1, 5);

INSERT INTO users_songs (user_id, song_id)
VALUES (2, 5);

INSERT INTO users_albums (user_id, album_id)
VALUES (2, 1);

INSERT INTO tags (name)
VALUES ('post-punk');

INSERT INTO tags (name)
VALUES ('shoegaze');

INSERT INTO tags (name)
VALUES ('ambient');

INSERT INTO artist_tags (artist_id, tag_id)
VALUES (1, 1);

INSERT INTO artist_tags (artist_id, tag_id)
VALUES (1, 2);

INSERT INTO album_tags (album_id, tag_id)
VALUES (2, 3);