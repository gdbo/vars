-- Add up migration script here
DELETE FROM role;
INSERT INTO role(`name`, `default`, `permissions`) VALUES ("User", 1, 2);
INSERT INTO role(`name`, `default`, `permissions`) VALUES ("Admin", 0, 30);
