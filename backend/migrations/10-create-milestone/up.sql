CREATE TABLE milestone (
  id INTEGER PRIMARY KEY,
  `name` VARCHAR(60) NOT NULL,
  `board_id` INTEGER,
  `board_type` VARCHAR(60)
)
