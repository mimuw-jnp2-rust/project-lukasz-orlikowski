CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  `username` VARCHAR(60) NOT NULL UNIQUE,
  `password` VARCHAR(60) NOT NULL
);
