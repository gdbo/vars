-- Add up migration script here
-- role
CREATE TABLE IF NOT EXISTS role (
  id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR(64) NOT NULL,
  `default` TINYINT(1) NOT NULL DEFAULT 0,
  permissions int NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `role_name` (`name`),
  KEY `role_default` (`default`)
);

-- user
CREATE TABLE IF NOT EXISTS user (
  id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR(64) NOT NULL,
  password_hash VARCHAR(128) NOT NULL,
  email VARCHAR(64) NOT NULL,
  role_id INT NOT NULL,
  avatar VARCHAR(128),
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_seen DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  deleted_at DATETIME,
  is_active TINYINT(1) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `user_name` (`name`),
  UNIQUE KEY `user_email` (`email`),
  CONSTRAINT `user_role_id` FOREIGN KEY (`role_id`) REFERENCES `role` (`id`)
);

-- category
CREATE TABLE IF NOT EXISTS category (
  id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR(64),
  description VARCHAR(128),
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  deleted_at DATETIME,
  PRIMARY KEY (`id`),
  UNIQUE KEY `category_name` (`name`)
);

-- tag
CREATE TABLE IF NOT EXISTS tag (
  id INT NOT NULL AUTO_INCREMENT,
  name VARCHAR(64),
  description VARCHAR(128),
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  deleted_at DATETIME,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tag_name` (`name`)
);

-- article
CREATE TABLE IF NOT EXISTS article (
  id INT NOT NULL AUTO_INCREMENT,
  title VARCHAR(256),
  slug VARCHAR(128),
  content TEXT,
  summary VARCHAR(256),
  cover VARCHAR(64),
  status TINYINT(8) NOT NULL,
  password VARCHAR(32),
  read_count INT NOT NULL DEFAULT 0,
  like_count INT NOT NULL DEFAULT 0,
  is_top TINYINT(1) NOT NULL DEFAULT 0,
  category_id INT NOT NULL,
  user_id INT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  deleted_at DATETIME,
  PRIMARY KEY (`id`),
  CONSTRAINT `article_author_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`),
  CONSTRAINT `article_category_id` FOREIGN KEY (`category_id`) REFERENCES `category` (`id`)
);

-- comment
CREATE TABLE IF NOT EXISTS comment (
  id INT NOT NULL AUTO_INCREMENT,
  content TEXT,
  article_id INT NOT NULL,
  user_id INT NOT NULL,
  like_count INT NOT NULL DEFAULT 0,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  CONSTRAINT `comment_article_id` FOREIGN KEY (`article_id`) REFERENCES `article` (`id`) ON DELETE CASCADE,
  CONSTRAINT `comment_user_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE
);

-- reply
CREATE TABLE IF NOT EXISTS reply (
  id INT NOT NULL AUTO_INCREMENT,
  content TEXT NOT NULL,
  user_id INT NOT NULL,
  comment_id INT DEFAULT NULL,
  reply_id INT DEFAULT NULL,
  reply_type TINYINT(1) NOT NULL,
  like_count INT NOT NULL DEFAULT 0,
  created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  CONSTRAINT `reply_user_id` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`) ON DELETE CASCADE,
  CONSTRAINT `reply_comment_id` FOREIGN KEY (`comment_id`) REFERENCES `comment` (`id`) ON DELETE CASCADE,
  CONSTRAINT `reply_reply_id` FOREIGN KEY (`reply_id`) REFERENCES `reply` (`id`) ON DELETE CASCADE
);

-- article_tag
CREATE TABLE IF NOT EXISTS article_tag (
  id INT NOT NULL AUTO_INCREMENT,
  article_id INT NOT NULL,
  tag_id INT NOT NULL,
  PRIMARY KEY (`id`),
  CONSTRAINT `at_article_id` FOREIGN KEY (`article_id`) REFERENCES `article` (`id`),
  CONSTRAINT `at_tag_id` FOREIGN KEY (`tag_id`) REFERENCES `tag` (`id`)
);
