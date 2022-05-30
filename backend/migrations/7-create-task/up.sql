CREATE TABLE task (
  id INTEGER PRIMARY KEY,
  `name` VARCHAR(60) NOT NULL,
  `list` INTEGER,
  `note` VARCHAR(200),
  `place` VARCHAR(60),
  `members` VARCHAR(200),
  `deadline` VARCHAR(60),
  `subtasks` VARCHAR(200),
   FOREIGN KEY(`list`) REFERENCES list(id) ON DELETE CASCADE
)
