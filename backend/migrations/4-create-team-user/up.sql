CREATE TABLE team_user (
   id INTEGER PRIMARY KEY,
  `user` INTEGER,
  `team` INTEGER,
  FOREIGN KEY(`user`) REFERENCES users(id),
  FOREIGN KEY (`team`) REFERENCES team(id),
  UNIQUE (`team`, `user`)
)
