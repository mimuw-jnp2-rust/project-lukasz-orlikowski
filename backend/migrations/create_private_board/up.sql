CREATE TABLE private_board (
  id INT(11) PRIMARY KEY,
  `name` VARCHAR(60) NOT NULL,
  `owner`INTEGER,
  FOREIGN KEY(`owner`) REFERENCES users(id)
)
