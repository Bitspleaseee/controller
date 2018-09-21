-- Your SQL goes here
CREATE TABLE `categories` (

  `id` INT NOT NULL AUTO_INCREMENT,
  `title` VARCHAR(45) NOT NULL,
  `description` TEXT NOT NULL,
  `hidden` BOOLEAN NOT NULL DEFAULT 0,

  PRIMARY KEY (`id`)
);
