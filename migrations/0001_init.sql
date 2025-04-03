CREATE TABLE IF NOT EXISTS users
(
id serial PRIMARY KEY,
created timestamptz DEFAULT CURRENT_TIMESTAMP,
email varchar UNIQUE,
id_string varchar UNIQUE,
display varchar
);

CREATE TABLE IF NOT EXISTS posts
(
id serial PRIMARY KEY,
author integer NOT NULL REFERENCES users(id),
title varchar NOT NULL,
publish timestamptz
);

CREATE TABLE IF NOT EXISTS translations
(
post_id integer REFERENCES posts(id) ON DELETE CASCADE,
language varchar NOT NULL,
title varchar NOT NULL,
description varchar,
PRIMARY KEY(post_id, language)
);

CREATE TABLE IF NOT EXISTS events
(
post_id integer PRIMARY KEY REFERENCES posts(id) ON DELETE CASCADE,
location varchar,
start_time timestamptz,
end_time timestamptz
);
