-- Your SQL goes here
CREATE TABLE bfp (
  id SERIAL PRIMARY KEY,
  bfp_hash VARCHAR NOT NULL,
  user_agent VARCHAR NOT NULL
)