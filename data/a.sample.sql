CREATE TABLE IF NOT EXISTS `a` (
  `doc_id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `media_id` int(10) unsigned NOT NULL DEFAULT '0',
  `poster_ip` decimal(39,0) unsigned NOT NULL DEFAULT '0',
  `num` int(10) unsigned NOT NULL,
  `subnum` int(10) unsigned NOT NULL,
  `thread_num` int(10) unsigned NOT NULL DEFAULT '0',
  `op` tinyint(1) NOT NULL DEFAULT '0',
  `timestamp` int(10) unsigned NOT NULL,
  `timestamp_expired` int(10) unsigned NOT NULL,
  `preview_orig` varchar(20) DEFAULT NULL,
  `preview_w` smallint(5) unsigned NOT NULL DEFAULT '0',
  `preview_h` smallint(5) unsigned NOT NULL DEFAULT '0',
  `media_filename` text,
  `media_w` smallint(5) unsigned NOT NULL DEFAULT '0',
  `media_h` smallint(5) unsigned NOT NULL DEFAULT '0',
  `media_size` int(10) unsigned NOT NULL DEFAULT '0',
  `media_hash` varchar(25) DEFAULT NULL,
  `media_orig` varchar(20) DEFAULT NULL,
  `spoiler` tinyint(1) NOT NULL DEFAULT '0',
  `deleted` tinyint(1) NOT NULL DEFAULT '0',
  `capcode` varchar(1) NOT NULL DEFAULT 'N',
  `email` varchar(100) DEFAULT NULL,
  `name` varchar(100) DEFAULT NULL,
  `trip` varchar(25) DEFAULT NULL,
  `title` varchar(100) DEFAULT NULL,
  `comment` text,
  `delpass` tinytext,
  `sticky` tinyint(1) NOT NULL DEFAULT '0',
  `locked` tinyint(1) NOT NULL DEFAULT '0',
  `poster_hash` varchar(8) DEFAULT NULL,
  `poster_country` varchar(2) DEFAULT NULL,
  `exif` text,
  PRIMARY KEY (`doc_id`),
  UNIQUE KEY `num_subnum_index` (`num`,`subnum`),
  KEY `thread_num_subnum_index` (`thread_num`,`num`,`subnum`),
  KEY `subnum_index` (`subnum`),
  KEY `op_index` (`op`),
  KEY `media_id_index` (`media_id`),
  KEY `media_hash_index` (`media_hash`),
  KEY `media_orig_index` (`media_orig`),
  KEY `name_trip_index` (`name`,`trip`),
  KEY `trip_index` (`trip`),
  KEY `email_index` (`email`),
  KEY `poster_ip_index` (`poster_ip`),
  KEY `timestamp_index` (`timestamp`)
) ENGINE=TokuDB AUTO_INCREMENT=260985878 DEFAULT CHARSET=utf8mb4;


﻿INSERT INTO `a` (`num`, `subnum`, `thread_num`, `op`, `timestamp`, `timestamp_expired`, `preview_orig`, `preview_w`, `preview_h`, `media_filename`, `media_w`, `media_h`, `media_size`, `media_hash`, `media_orig`, `spoiler`, `deleted`, `capcode`, `email`, `name`, `trip`, `title`, `comment`, `sticky`, `locked`, `poster_hash`, `poster_country`, `exif`) VALUES
(131902474, 0, 131902474, 1, 1444189276, 0, '1444203676799s.jpg', 170, 250, '11884l.jpg', 307, 450, 78268, 'JiWLc2TDFyKAqdxf7KsA7A==', '1444203676799.jpg', 0, 0, 'N', NULL, 'Anonymous', NULL, 'Ah! My Goddess', 'Is this actually worth watching or is it just another shitty harem?', 0, 0, NULL, NULL, NULL),
(131902553, 0, 131902474, 0, 1444189481, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'Dunno but the doujins are nice.', 0, 0, NULL, NULL, NULL),
(131902888, 0, 131902474, 0, 1444190228, 0, '1444204628101s.jpg', 125, 93, 'Ah! My Goddess TV - 02 - [AonE-AnY][DTV][640x480][XviD][MP3 CBR][EF89D6D2].avi_snapshot_09.13_[2015.10.06_21.51.43].jpg', 640, 480, 39796, 'Y/dVBVLAfPoprgJLvSsWSw==', '1444204628101.jpg', 1, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'It\'s a nice romcom anime about the love between a forever alone loser and a magical android goddess conquering all. \n\n[spoiler]But when you realize that the entire series is about her being magically forced to love somebody against her will, the show is kinda interesting.\n\nI mean just look at this snap from the moment after he makes the wish she is there to grant. Look at those eyes, she knows what is going to happen and is powerless to do anything about it before her brain is literally rewritten to make her love him.[/spoiler]', 0, 0, NULL, NULL, NULL),
(131903259, 0, 131902474, 0, 1444191084, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902888\nHis wish was that she would stay by his side, love is not necessarily involved and certainly not forced.', 0, 0, NULL, NULL, NULL),
(131903263, 0, 131902474, 0, 1444191104, 0, '1444205504518s.jpg', 90, 125, 'true form.jpg', 900, 1248, 131014, 'O9oF0H3ze5+RPzVHVCBORw==', '1444205504518.jpg', 1, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\nFeels different from today\'s harems. Which often are battle-harems anyway.\n\nThe other girls are just wingmen/freeloaders in a sense, they aren\'t all that interested in him. Although there occasionally are advances on K1 belldandy is pretty much his designated girlfriend.\n\nOne thing to consider is that it\'s very slow-paced. I don\'t know how many chapters it took them to even took them to manage an \"i love you\" (Ai, not daisuki bullshitt) or a kiss that\'s not under duress/hypnosis/whatever. The manga has been released over several decades, that\'s how slow it is.\n\n>>131902888\n>magical android goddess\nor eldritch abominations. depending on your point of view', 0, 0, NULL, NULL, NULL),
(131903721, 0, 131902474, 0, 1444192374, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\nWatch it and Read it, it\'s not a shitty harem. This author draws the best motorcycles, engines, cars and such. Series for mechanicfags.', 0, 0, NULL, NULL, NULL),
(131903767, 0, 131902474, 0, 1444192517, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131903259\n>love is not necessarily involved and certainly not forced.\n\neither way it\'s still Stockholm syndrome', 0, 0, NULL, NULL, NULL),
(131903771, 0, 131902474, 0, 1444192544, 0, '1444206944391s.jpg', 118, 125, 'Belldandy.jpg', 340, 360, 28942, '1zaiyRlGn4P3BGqx6C/ACA==', '1444206944391.jpg', 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'It\'s one of my comfort animes. It\'s also not harem.', 0, 0, NULL, NULL, NULL),
(131903835, 0, 131902474, 0, 1444192690, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902888\nDon\'t forget she gets knocked out from transmitting the wish and immediately afterwards rushes in panic to call Almighty to confirm if the nigger is really serious about the approval.', 0, 0, NULL, NULL, NULL),
(131903923, 0, 131902474, 0, 1444192892, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131903835\nOh, I didn\'t forget. But by then she had already been changed, all that was left for her to do was resign herself to her fate.', 0, 0, NULL, NULL, NULL),
(131903954, 0, 131902474, 0, 1444192992, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'what? didn\'t belldandy actually meet k1 a long time ago and she actually loves him?', 0, 0, NULL, NULL, NULL),
(131903992, 0, 131902474, 0, 1444193088, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131903923\nAlso it was retconned, she had already fallen for K1 since she had been observing him as wish applicant. Fuck the ending was shit. All that good stuff about gods and devils being horrible lovecraftesque abominations only being a side remark (and their human forms naturally being just to not alert the mortals just like with angels in the Bible).', 0, 0, NULL, NULL, NULL),
(131904020, 0, 131902474, 0, 1444193164, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'This show was made for lonely boys who wanted their mother as a girlfriend.', 0, 0, NULL, NULL, NULL),
(131904086, 0, 131902474, 0, 1444193327, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131903771\n>It\'s also not harem.\nI think every girl wanted at least once into mcs pants. Be it through spell, puberty or jealousy, but this power fantasy still had its place present.', 0, 0, NULL, NULL, NULL),
(131904093, 0, 131902474, 0, 1444193366, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131903992\nWhy did she secretly program the cockblock again?', 0, 0, NULL, NULL, NULL),
(131904112, 0, 131902474, 0, 1444193419, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131904020\nSo is there good milf action?', 0, 0, NULL, NULL, NULL),
(131904123, 0, 131902474, 0, 1444193469, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131904093\nBecause god-mortal abominations are bad m\'kay?', 0, 0, NULL, NULL, NULL),
(131904128, 0, 131902474, 0, 1444193483, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\n\nIt proceeds at a relaxed pace and all the girls are qt.\n\nI recommend you watch it.', 0, 0, NULL, NULL, NULL),
(131904169, 0, 131902474, 0, 1444193621, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131904112\nNo.. That\'s why there is no kissing or sex. Eww, you want to sleep with your mother girlfriend?', 0, 0, NULL, NULL, NULL),
(131904195, 0, 131902474, 0, 1444193695, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131904112\n>only like 1 doujin where loli form Hild gets banged\n>0 milf Hild \n\nGod Almighty why do they hate Hild?', 0, 0, NULL, NULL, NULL),
(131904957, 0, 131902474, 0, 1444195776, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 1, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131904195\nhe doesn\'t. evidence: urd', 0, 0, NULL, NULL, NULL),
(131905676, 0, 131902474, 0, 1444198061, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\n\nIf you can watch k-on, gochiusa or chu2, you can watch this and the same feels will be felt.', 0, 0, NULL, NULL, NULL),
(131905734, 0, 131902474, 0, 1444198216, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131904169\n\nActually, there is some kissing.', 0, 0, NULL, NULL, NULL),
(131906763, 0, 131902474, 0, 1444201507, 0, '1444215907015s.jpg', 86, 125, 'K1WakesUpAtLongLast.png', 1112, 1600, 287864, 'saczBkoi02TXNjiBdcl3fw==', '1444215907015.png', 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, NULL, 0, 0, NULL, NULL, NULL),
(131906851, 0, 131902474, 0, 1444201697, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\n\nAnd God himself intervened', 0, 0, NULL, NULL, NULL),
(131906939, 0, 131902474, 0, 1444201965, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'This would have been entirely forgettable if it came out a decade or two later.', 0, 0, NULL, NULL, NULL),
(131906993, 0, 131902474, 0, 1444202117, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131906763\n\nWell, at least they had a happy ending. I was kinda fearing a grimdark end.', 0, 0, NULL, NULL, NULL),
(131907260, 0, 131902474, 0, 1444202786, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902888\nWasn\'t a huge shitstorm some years ago that was caused pressingly because of it? That Beldandy was a Slut or that he was forcing MC to love her or she was being force to. I don\'t remember', 0, 0, NULL, NULL, NULL),
(131908372, 0, 131902474, 0, 1444205508, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131906993\nIt ended? The first OVA series was one of my first animus when I was a kid. How did it end?', 0, 0, NULL, NULL, NULL),
(131908400, 0, 131902474, 0, 1444205578, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'There was a reveal MC\'s sense of lust was neutered in his brain by Ygrdassil in order to comply with his wish / or after making it. \nEither way the goddesses were revealed to have known about it all along. \nSee page above.', 0, 0, NULL, NULL, NULL),
(131908423, 0, 131902474, 0, 1444205639, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131908372\n\nThe manga ended.', 0, 0, NULL, NULL, NULL),
(131908612, 0, 131902474, 0, 1444206120, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131906993\nDid they fug?', 0, 0, NULL, NULL, NULL),
(131908760, 0, 131902474, 0, 1444206500, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131907260\nIt was so huge shit storm it spread to /tg/.', 0, 0, NULL, NULL, NULL),
(131908797, 0, 131902474, 0, 1444206590, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\nIt\'s worth watching if you\'re into letting other men fuck your girl', 0, 0, NULL, NULL, NULL),
(131908813, 0, 131902474, 0, 1444206628, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131907260\nBasically when Keeichi made the wish to keep Belldandy by her side there was also a hidden, secret clause no one bothered to tell him about: that in doing so he would also have his sexual desire completely neutered as to prevent mortal-goddess babies to be made between them.\nBelldandy knew about this but said nothing to him, feeling happy to have this guy dote her while at the same time easily avoiding any awkward sexual pressure.\nAll the other freeloader goddesess knew as well (see >>131906763 ) but neither of them bothered to tell him that he\'d been magically neutered without knowledge nor consent.\n\nTl;dr we\'ve been for many years complaining about what a complete pussy K1 was to never do anything real, only to discover that he\'d been castrated against his will.', 0, 0, NULL, NULL, NULL),
(131908870, 0, 131902474, 0, 1444206759, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131908813\nThis is legitimately one of my favorite twists in manga/anime.', 0, 0, NULL, NULL, NULL),
(131908913, 0, 131902474, 0, 1444206865, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131908870\n\nToo bad it was rushed and felt forcefully bolted on.', 0, 0, NULL, NULL, NULL),
(131908923, 0, 131902474, 0, 1444206878, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\n\nIt\'s one of \"the\" harems. You can\'t really compare it to the LN shit we have today.', 0, 0, NULL, NULL, NULL),
(131909144, 0, 131902474, 0, 1444207386, 0, '1444221786613s.jpg', 87, 125, 'Oh My Goddess!3.jpg', 850, 1213, 619642, 'dqDO8nH3W6cuWF7iA6yUSQ==', '1444221786613.jpg', 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131908813\n\nI feel pretty bad about that for him. How did he react to it?', 0, 0, NULL, NULL, NULL),
(131909159, 0, 131902474, 0, 1444207418, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131908923\nAlso better read the manga since it has way more and better shit. The only thing going on in the anime is Wakamoto\'s brief appearance.', 0, 0, NULL, NULL, NULL),
(131909238, 0, 131902474, 0, 1444207603, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131909144\n\nAlmost mindbroke. Her love saved him, though.', 0, 0, NULL, NULL, NULL),
(131909239, 0, 131902474, 0, 1444207606, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131909144\nOh no how horrible I\'ve been lied to for years and years I forgive you lets get married.', 0, 0, NULL, NULL, NULL),
(131909242, 0, 131902474, 0, 1444207608, 0, '1444222008407s.jpg', 125, 125, 'Belldandy_(Manga)_cropped_-_v1_p7[1].png', 250, 250, 54491, 'PZ8dNvf5LQlcig02oo2/dA==', '1444222008407.png', 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131909159\nManga also has this. Ah My Goddess? More like Ayy My Goddess.', 0, 0, NULL, NULL, NULL),
(131909287, 0, 131902474, 0, 1444207703, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131909242\nHey the manga is a wonderful time travel diary through the late 1980s to the early 2010s.', 0, 0, NULL, NULL, NULL),
(131909289, 0, 131902474, 0, 1444207706, 0, '1444222106979s.jpg', 83, 125, 'URD.jpg', 300, 450, 29383, 'zv7fv81F8DOc+quwaZbirA==', '1444222106979.jpg', 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, 'Urd is best girl. If she\'d been the one to answer the call shit woulda been super interesting.', 0, 0, NULL, NULL, NULL),
(131909474, 0, 131902474, 0, 1444208116, 0, '1444222516769s.jpg', 125, 125, '1358581183120.gif', 342, 342, 511827, '7YliOV9EYcT+Mvct4apuPQ==', '1444222516769.gif', 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131908813\nHOLY SHIT THAT\'S AMAZING.\nFinally! An in-story explanation for a beta harem MC. Genius. Absolutely genius.', 0, 0, NULL, NULL, NULL),
(131909513, 0, 131902474, 0, 1444208221, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131909289\n\nThere\'s a few quality fanfics with just that as a premise.', 0, 0, NULL, NULL, NULL),
(131909530, 0, 131902474, 0, 1444208261, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131909513\nDavner hasn\'t updated Haloes in ages, dont remind me!', 0, 0, NULL, NULL, NULL),
(131912973, 0, 131902474, 0, 1444214986, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131902474\nNot worth watching.', 0, 0, NULL, NULL, NULL),
(131913724, 0, 131902474, 0, 1444216129, 0, NULL, 0, 0, NULL, 0, 0, 0, NULL, NULL, 0, 0, 'N', NULL, 'Anonymous', NULL, NULL, '>>131908813\nHidden clauses,just like real corporations.', 0, 0, NULL, NULL, NULL);
