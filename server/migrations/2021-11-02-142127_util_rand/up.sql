-- Your SQL goes here-- Your SQL goes here
CREATE TABLE `util_rand`
(
    `id`         int(11)                             NOT NULL AUTO_INCREMENT,
    `value1`     int(11)                             NOT NULL,
    `value2`     int(11)                             NOT NULL,
    `value3`     int(11)                             NOT NULL,
    `value4`     int(11)                             NOT NULL,
    `value5`     int(11)                             NOT NULL,
    `value6`     int(11)                             NOT NULL,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_unicode_ci;