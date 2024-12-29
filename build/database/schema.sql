
CREATE TABLE IF NOT EXISTS `users` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

CREATE TABLE IF NOT EXISTS `prompt_formats` (
  `id` varchar(32) NOT NULL AUTO_INCREMENT, /* TODO: Use UUIDs in the form of strings */
  `name` varchar(255) NOT NULL,
  `prompt` text NOT NULL,
  `prompt_args` text NOT NULL, 
  `created_at` datetime NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8;

/* TODO: Waypoints table */
