CREATE TABLE team_board (
  id INTEGER PRIMARY KEY,
  `name` VARCHAR(60) NOT NULL,
  `owner`INTEGER,
  FOREIGN KEY(`owner`) REFERENCES team(id)
)
