CREATE TABLE log (
  id INTEGER PRIMARY KEY,
  `name` VARCHAR(60) NOT NULL,
  `list` INTEGER,
  `note` VARCHAR(200),
  `place` VARCHAR(60),
  `members` VARCHAR(200),
  `timestamp` VARCHAR(60),
  `action` VARCHAR(60),
  `task_id` INTEGER,
  `deadline` VARCHAR(60),
  `subtasks` VARCHAR(200),
  `points` INTEGER,
  `tags` VARCHAR(200),
   FOREIGN KEY(`list`) REFERENCES list(id) ON DELETE CASCADE,
   FOREIGN KEY(`task_id`) REFERENCES task(id) ON DELETE CASCADE
)
