CREATE TABLE people (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  born_at DATE,
  updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('people');
