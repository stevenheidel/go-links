CREATE TABLE links (
  id SERIAL PRIMARY KEY,
  src VARCHAR NOT NULL,
  dest VARCHAR NOT NULL,
  hits INTEGER DEFAULT 0
);

CREATE UNIQUE INDEX src_idx ON links (src);
CREATE INDEX dest_idx ON links (dest);
CREATE INDEX hits_idx ON links (hits);
