-- import to SQLite by running: sqlite3.exe db.sqlite3 -init sqlite.sql

PRAGMA journal_mode = MEMORY;
PRAGMA synchronous = OFF;
PRAGMA foreign_keys = OFF;
PRAGMA ignore_check_constraints = OFF;
PRAGMA auto_vacuum = NONE;
PRAGMA secure_delete = OFF;
BEGIN TRANSACTION;

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";

CREATE TABLE `comments` (
`id` INTEGER NOT NULL,
`post_id` INTEGER NOT NULL,
`owner_id` INTEGER NOT NULL,
`body` TEXT NOT NULL
);
INSERT INTO `comments` (`id`, `post_id`, `owner_id`, `body`) VALUES
(4, 15, 5, 'hello'),
(5, 15, 5, 'there'),
(6, 15, 5, 'According to all known laws of aviation, there is no way a bee should be able to fly. Its wings are too small to get its fat little body off the ground. The bee, of course, flies anyway because bees don''t care what humans think is impossible. Yellow, black. Yellow, black. Yellow, black. Yellow, black. Ooh, black and yellow! Let''s shake it up a little. Barry! Breakfast is ready! Coming! Hang on a second. Hello? Barry? Adam? Can you believe this is happening? I can''t. I''ll pick you up. Looking sharp. Use the stairs, Your father paid good money for those. Sorry. I''m excited. Here''s the graduate. We''re very proud of you, son. A perfect report card, all B''s. Very proud. Ma! I got a thing going here. You got lint on your fuzz. Ow! That''s me! Wave to us! We''ll be in row 118,000. Bye! Barry, I told you, stop flying in the house! Hey, Adam. Hey, Barry. Is that fuzz gel? A little. Special day, graduation. Never thought I''d make it. Three days grade school, three days high school. Those were awkw'),
(8, 6, 5, 'Nice Picture Dawg'),
(9, 16, 5, 'Love was here'),
(10, 16, 5, 'Posted from my iPhone '),
(11, 16, 5, 'Bzz');

CREATE TABLE `posts` (
`id` INTEGER NOT NULL,
`owner_id` INTEGER NOT NULL,
`title` TEXT NOT NULL,
`body` TEXT NOT NULL,
`image` TEXT NOT NULL
);
INSERT INTO `posts` (`id`, `owner_id`, `title`, `body`, `image`) VALUES
(2, 4, 'soldier', '', 'd8f34073-12f2-4c63-9276-7dce57742b7f.png'),
(3, 2, 'space', '', '12f2702e-b911-4132-acaa-4b80b3bc3734.png'),
(5, 5, 'Quilt', '', '0a6b3e4b-af70-4307-a761-1c9d4ffb51db.png'),
(6, 6, 'NMS art', '', 'e413a030-7887-4cd2-b447-ce16d307892d.png'),
(8, 7, '?????', '', 'ed6f7179-c012-4719-a633-478426372db4.png'),
(10, 7, 'Mondis caught in action', '', '699ca946-ab19-4c18-ab9b-96b5b72aef0e.png'),
(15, 7, 'Ace of Spades', 'Motörhead ', '5395fb7b-d763-4ed7-b1f1-6bacf11cc217.png'),
(16, 5, 'Bee', 'According to all known laws of aviation, there is no way a bee should be able to fly. Its wings are too small to get its fat little body off the ground. The bee, of course, flies anyway because bees don''t care what humans think is impossible. Yellow, black. Yellow, black. Yellow, black. Yellow, black. Ooh, black and yellow! Let''s shake it up a little. Barry! Breakfast is ready! Coming! Hang on a second. Hello? Barry? Adam? Can you believe this is happening? I can''t. I''ll pick you up. Looking sharp. Use the stairs, Your father paid good money for those. Sorry. I''m excited. Here''s the graduate. We''re very proud of you, son. A perfect report card, all B''s. Very proud. Ma! I got a thing going here. You got lint on your fuzz. Ow! That''s me! Wave to us! We''ll be in row 118,000. Bye! Barry, I told you, stop flying in the house! Hey, Adam. Hey, Barry. Is that fuzz gel? A little. Special day, graduation. Never thought I''d make it. Three days grade school, three days high school. Those were awkward. Three days college. I''m glad I took a day and hitchhiked around The Hive. You did come back different. Hi, Barry. Artie, growing a mustache? Looks good. Hear about Frankie? Yeah. You going to the funeral? No, I''m not going. Everybody knows, sting someone, you die. Don''t waste it on a squirrel. Such a hothead. I guess he could have just gotten out of the way. I love this incorporating an amusement park into our day. That''s why we don''t need vacations. Boy, quite a bit of pomp under the circumstances. Well, Adam, today we are men. We are! Bee-men. Amen! Hallelujah! Students, faculty, distinguished bees, please welcome Dean Buzzwell. Welcome, New Hive City graduating class of 9:15. That concludes our ceremonies And begins your career at Honex Industries! Will we pick our job today? I heard it''s just orientation. Heads up! Here we go. Keep your hands and antennas inside the tram at all times. Wonder what it''ll be like? A little scary. Welcome to Honex, a division of Honesco and a part', '6313f6d7-f49d-459a-a096-ec384e0893b1.png');

CREATE TABLE `users` (
`id` INTEGER NOT NULL,
`username` TEXT NOT NULL,
`password` TEXT NOT NULL,
`avatar` TEXT NOT NULL
);
INSERT INTO `users` (`id`, `username`, `password`, `avatar`) VALUES
(2, 'test', '$2b$10$yu62Xiu20eMA68XX21MQIuwLQpCwbH5u0SNvF8J3BSal1L./5i6iy', ''),
(4, 'per', '$2b$10$exqDHJ1XjPJujtcuQExj7.tgYFEud3cbYhAjWjKElWNKjTfZnM/Ta', ''),
(5, 'Board', '$2b$10$EByo1d2wLoH1RlCkj/.uUuTZO5/QreMr9w6/mysvHloN92at1LY1.', ''),
(6, 'Odder', '$2b$10$XAAROHobryRND3Aif8Iat.bPg3oRYLsK4TwsYQlEWkczkwVv3uDBG', ''),
(7, 'boo', '$2b$10$2yaZr4vgR5AB2c2kfHy00.fRNAE7WuYG0ODYoMpix4jMubtg.RcwW', '');
ALTER TABLE `comments`
ADD PRIMARY KEY (`id`);
ALTER TABLE `posts`
ADD PRIMARY KEY (`id`);
ALTER TABLE `users`
ADD PRIMARY KEY (`id`);
ALTER TABLE `comments`
MODIFY `id` INTEGER NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=12;
ALTER TABLE `posts`
MODIFY `id` INTEGER NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=17;
ALTER TABLE `users`
MODIFY `id` INTEGER NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;
COMMIT;





COMMIT;
PRAGMA ignore_check_constraints = ON;
PRAGMA foreign_keys = ON;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
