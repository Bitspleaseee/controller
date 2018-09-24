CREATE TABLE users (

  id INT UNSIGNED NOT NULL,
  username VARCHAR(20) NOT NULL,
  description VARCHAR(255) NULL,
  avatar VARCHAR(36) NULL,

  PRIMARY KEY (id)
);
