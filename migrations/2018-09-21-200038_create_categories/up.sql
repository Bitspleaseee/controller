-- Your SQL goes here
CREATE TABLE `categories` (

  `id` INT NOT NULL AUTO_INCREMENT,
  `title` VARCHAR(20) NOT NULL,
  `description` VARCHAR(255) NULL,
  `hidden` BOOLEAN NOT NULL DEFAULT 0,

  PRIMARY KEY (`id`)
);
