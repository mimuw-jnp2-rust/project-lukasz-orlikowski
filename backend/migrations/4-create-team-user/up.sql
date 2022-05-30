CREATE TABLE team_user (
   id INTEGER PRIMARY KEY,
  `user` INTEGER,
  `team` INTEGER,
  FOREIGN KEY(`user`) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (`team`) REFERENCES team(id) ON DELETE CASCADE,
  UNIQUE (`team`, `user`)
)
