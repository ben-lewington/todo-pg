CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE SCHEMA todos;

CREATE TABLE todos.main (
    id         serial primary key
  , ident      uuid not null default uuid_generate_v4()
  , name       varchar(255) not null
  , completed  timestamp with time zone null
);
