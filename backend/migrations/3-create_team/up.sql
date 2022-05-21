CREATE TABLE team (
  id INTEGER PRIMARY KEY,
  `name` VARCHAR(60) NOT NULL,
  `owner` INTEGER,
  FOREIGN KEY(`owner`) REFERENCES users(id),
  UNIQUE (`name`, `owner`)
)
