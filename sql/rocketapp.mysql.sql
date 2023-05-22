-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Värd: 127.0.0.1
-- Tid vid skapande: 22 maj 2023 kl 14:46
-- Serverversion: 10.4.27-MariaDB
-- PHP-version: 8.2.0

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Databas: `rocketapp`
--

-- --------------------------------------------------------

--
-- Tabellstruktur `comments`
--

CREATE TABLE `comments` (
  `id` int(11) NOT NULL,
  `post_id` int(11) NOT NULL,
  `owner_id` int(11) NOT NULL,
  `body` varchar(1000) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumpning av Data i tabell `comments`
--

INSERT INTO `comments` (`id`, `post_id`, `owner_id`, `body`) VALUES
(4, 15, 5, 'hello'),
(5, 15, 5, 'there'),
(6, 15, 5, 'According to all known laws of aviation, there is no way a bee should be able to fly. Its wings are too small to get its fat little body off the ground. The bee, of course, flies anyway because bees don\'t care what humans think is impossible. Yellow, black. Yellow, black. Yellow, black. Yellow, black. Ooh, black and yellow! Let\'s shake it up a little. Barry! Breakfast is ready! Coming! Hang on a second. Hello? Barry? Adam? Can you believe this is happening? I can\'t. I\'ll pick you up. Looking sharp. Use the stairs, Your father paid good money for those. Sorry. I\'m excited. Here\'s the graduate. We\'re very proud of you, son. A perfect report card, all B\'s. Very proud. Ma! I got a thing going here. You got lint on your fuzz. Ow! That\'s me! Wave to us! We\'ll be in row 118,000. Bye! Barry, I told you, stop flying in the house! Hey, Adam. Hey, Barry. Is that fuzz gel? A little. Special day, graduation. Never thought I\'d make it. Three days grade school, three days high school. Those were awkw'),
(8, 6, 5, 'Nice Picture Dawg'),
(9, 16, 5, 'Love was here'),
(10, 16, 5, 'Posted from my iPhone '),
(11, 16, 5, 'Bzz');

-- --------------------------------------------------------

--
-- Tabellstruktur `posts`
--

CREATE TABLE `posts` (
  `id` int(11) NOT NULL,
  `owner_id` int(11) NOT NULL,
  `title` varchar(100) NOT NULL,
  `body` varchar(2000) NOT NULL,
  `image` varchar(100) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumpning av Data i tabell `posts`
--

INSERT INTO `posts` (`id`, `owner_id`, `title`, `body`, `image`) VALUES
(2, 4, 'soldier', '', 'd8f34073-12f2-4c63-9276-7dce57742b7f.png'),
(3, 2, 'space', '', '12f2702e-b911-4132-acaa-4b80b3bc3734.png'),
(5, 5, 'Quilt', '', '0a6b3e4b-af70-4307-a761-1c9d4ffb51db.png'),
(6, 6, 'NMS art', '', 'e413a030-7887-4cd2-b447-ce16d307892d.png'),
(8, 7, '?????', '', 'ed6f7179-c012-4719-a633-478426372db4.png'),
(10, 7, 'Mondis caught in action', '', '699ca946-ab19-4c18-ab9b-96b5b72aef0e.png'),
(15, 7, 'Ace of Spades', 'Motörhead ', '5395fb7b-d763-4ed7-b1f1-6bacf11cc217.png'),
(16, 5, 'Bee', 'According to all known laws of aviation, there is no way a bee should be able to fly. Its wings are too small to get its fat little body off the ground. The bee, of course, flies anyway because bees don\'t care what humans think is impossible. Yellow, black. Yellow, black. Yellow, black. Yellow, black. Ooh, black and yellow! Let\'s shake it up a little. Barry! Breakfast is ready! Coming! Hang on a second. Hello? Barry? Adam? Can you believe this is happening? I can\'t. I\'ll pick you up. Looking sharp. Use the stairs, Your father paid good money for those. Sorry. I\'m excited. Here\'s the graduate. We\'re very proud of you, son. A perfect report card, all B\'s. Very proud. Ma! I got a thing going here. You got lint on your fuzz. Ow! That\'s me! Wave to us! We\'ll be in row 118,000. Bye! Barry, I told you, stop flying in the house! Hey, Adam. Hey, Barry. Is that fuzz gel? A little. Special day, graduation. Never thought I\'d make it. Three days grade school, three days high school. Those were awkward. Three days college. I\'m glad I took a day and hitchhiked around The Hive. You did come back different. Hi, Barry. Artie, growing a mustache? Looks good. Hear about Frankie? Yeah. You going to the funeral? No, I\'m not going. Everybody knows, sting someone, you die. Don\'t waste it on a squirrel. Such a hothead. I guess he could have just gotten out of the way. I love this incorporating an amusement park into our day. That\'s why we don\'t need vacations. Boy, quite a bit of pomp under the circumstances. Well, Adam, today we are men. We are! Bee-men. Amen! Hallelujah! Students, faculty, distinguished bees, please welcome Dean Buzzwell. Welcome, New Hive City graduating class of 9:15. That concludes our ceremonies And begins your career at Honex Industries! Will we pick our job today? I heard it\'s just orientation. Heads up! Here we go. Keep your hands and antennas inside the tram at all times. Wonder what it\'ll be like? A little scary. Welcome to Honex, a division of Honesco and a part', '6313f6d7-f49d-459a-a096-ec384e0893b1.png');

-- --------------------------------------------------------

--
-- Tabellstruktur `users`
--

CREATE TABLE `users` (
  `id` int(11) NOT NULL,
  `username` varchar(30) NOT NULL,
  `password` varchar(128) NOT NULL,
  `avatar` varchar(100) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

--
-- Dumpning av Data i tabell `users`
--

INSERT INTO `users` (`id`, `username`, `password`, `avatar`) VALUES
(2, 'test', '$2b$10$yu62Xiu20eMA68XX21MQIuwLQpCwbH5u0SNvF8J3BSal1L./5i6iy', ''),
(4, 'per', '$2b$10$exqDHJ1XjPJujtcuQExj7.tgYFEud3cbYhAjWjKElWNKjTfZnM/Ta', ''),
(5, 'Board', '$2b$10$EByo1d2wLoH1RlCkj/.uUuTZO5/QreMr9w6/mysvHloN92at1LY1.', ''),
(6, 'Odder', '$2b$10$XAAROHobryRND3Aif8Iat.bPg3oRYLsK4TwsYQlEWkczkwVv3uDBG', ''),
(7, 'boo', '$2b$10$2yaZr4vgR5AB2c2kfHy00.fRNAE7WuYG0ODYoMpix4jMubtg.RcwW', '');

--
-- Index för dumpade tabeller
--

--
-- Index för tabell `comments`
--
ALTER TABLE `comments`
  ADD PRIMARY KEY (`id`);

--
-- Index för tabell `posts`
--
ALTER TABLE `posts`
  ADD PRIMARY KEY (`id`);

--
-- Index för tabell `users`
--
ALTER TABLE `users`
  ADD PRIMARY KEY (`id`);

--
-- AUTO_INCREMENT för dumpade tabeller
--

--
-- AUTO_INCREMENT för tabell `comments`
--
ALTER TABLE `comments`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=12;

--
-- AUTO_INCREMENT för tabell `posts`
--
ALTER TABLE `posts`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=17;

--
-- AUTO_INCREMENT för tabell `users`
--
ALTER TABLE `users`
  MODIFY `id` int(11) NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
