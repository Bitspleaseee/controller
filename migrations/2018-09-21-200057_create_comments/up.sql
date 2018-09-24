CREATE TABLE comments (

  id INT UNSIGNED NOT NULL AUTO_INCREMENT,
  thread_id INT UNSIGNED NOT NULL,
  parent_id INT UNSIGNED NULL,
  user_id INT UNSIGNED NOT NULL,
  content TEXT NOT NULL,
  hidden BOOLEAN NOT NULL DEFAULT 0,

  PRIMARY KEY (id),

  FOREIGN KEY (thread_id)
    REFERENCES threads(id),

  FOREIGN KEY (parent_id)
    REFERENCES comments(id),

  FOREIGN KEY (user_id)
    REFERENCES users(id)
);
