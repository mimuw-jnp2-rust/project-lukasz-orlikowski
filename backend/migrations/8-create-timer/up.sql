CREATE TABLE timer (
  id INTEGER PRIMARY KEY,
  `name` VARCHAR(60) NOT NULL,
  `user_id` INTEGER,
  `status` VARCHAR(60),
  `time` INTEGER,
  `start` INTEGER,
   FOREIGN KEY(`user_id`) REFERENCES users(id) ON DELETE CASCADE
)
