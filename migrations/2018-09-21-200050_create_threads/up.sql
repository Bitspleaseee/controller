CREATE TABLE `threads` (

  `id` INT NOT NULL AUTO_INCREMENT,
  `category_id` INT NOT NULL,
  `user_id` INT NOT NULL,
  `title` VARCHAR(45) NOT NULL,
  `description` TEXT NOT NULL,
  `timestamp` DATETIME NOT NULL DEFAULT NOW(),
  `hidden` BOOLEAN NOT NULL DEFAULT 0,

  PRIMARY KEY (`id`),

  FOREIGN KEY (category_id)
    REFERENCES categories(id),

  FOREIGN KEY (user_id)
    REFERENCES users(id)
);