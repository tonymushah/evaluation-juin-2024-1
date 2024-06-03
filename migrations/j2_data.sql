-- Insertion des catégories
INSERT INTO categorie (designation) VALUES 
('Cyclisme'), 
('VTT'), 
('Course à pied'), 
('Natation'), 
('Triathlon'), 
('Sprint'), 
('Marathon'), 
('Trail');

-- Insertion des coureurs
INSERT INTO coureur (nom, genre, dtn) VALUES 
('Alice', 1, '1990-05-14'),
('Bob', 2, '1988-11-23'),
('Charlie', 1, '1995-07-30'),
('David', 2, '1992-01-15'),
('Eva', 1, '1994-02-28'),
('Frank', 2, '1991-12-12'),
('Grace', 1, '1993-07-07'),
('Hank', 2, '1989-03-23');

-- Insertion des équipes
INSERT INTO equipe (pseudo, mot_passe, nom) VALUES 
('TeamA', 'passA', 'Equipe Alpha'),
('TeamB', 'passB', 'Equipe Beta'),
('TeamC', 'passC', 'Equipe Gamma'),
('TeamD', 'passD', 'Equipe Delta'),
('TeamE', 'passE', 'Equipe Epsilon');

-- Insertion des relations coureur_categorie
INSERT INTO coureur_categorie (categorie, coureur) VALUES 
((SELECT id_categorie FROM categorie WHERE designation = 'Cyclisme'), 1),
((SELECT id_categorie FROM categorie WHERE designation = 'VTT'), 2),
((SELECT id_categorie FROM categorie WHERE designation = 'Course à pied'), 3),
((SELECT id_categorie FROM categorie WHERE designation = 'Natation'), 4),
((SELECT id_categorie FROM categorie WHERE designation = 'Triathlon'), 5),
((SELECT id_categorie FROM categorie WHERE designation = 'Sprint'), 6),
((SELECT id_categorie FROM categorie WHERE designation = 'Marathon'), 7),
((SELECT id_categorie FROM categorie WHERE designation = 'Trail'), 8);

-- Insertion des relations equipe_coureur
INSERT INTO equipe_coureur (coureur, equipe) VALUES 
(1, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Alpha')),
(2, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Beta')),
(3, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Gamma')),
(4, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Delta')),
(5, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Epsilon')),
(6, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Alpha')),
(7, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Beta')),
(8, (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Gamma'));

-- Insertion des étapes
INSERT INTO etape (longueur, nom, nb_coureur_par_equipe, depart, finished) VALUES 
(5.0, 'Etape 1', 3, '2024-06-01 08:00:00', '2024-06-01 12:00:00'),
(10.0, 'Etape 2', 3, '2024-06-02 08:00:00', '2024-06-02 12:00:00'),
(15.0, 'Etape 3', 3, '2024-06-03 08:00:00', '2024-06-03 12:00:00'),
(20.0, 'Etape 4', 4, '2024-06-04 08:00:00', '2024-06-04 12:00:00'),
(25.0, 'Etape 5', 4, '2024-06-05 08:00:00', '2024-06-05 12:00:00'),
(30.0, 'Etape 6', 4, '2024-06-06 08:00:00', '2024-06-06 12:00:00');

-- Insertion des temps des coureurs
INSERT INTO temps_coureur (temps, points, equipe_coureur, etape) VALUES 
(300, 10, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 1 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Alpha')), 1),
(320, 8, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 2 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Beta')), 1),
(340, 6, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 3 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Gamma')), 1),
(360, 12, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 4 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Delta')), 2),
(380, 10, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 5 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Epsilon')), 2),
(400, 8, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 6 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Alpha')), 2),
(420, 6, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 7 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Beta')), 3),
(440, 4, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 8 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Gamma')), 3),
(460, 12, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 1 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Alpha')), 3),
(480, 10, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 2 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Beta')), 4),
(500, 8, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 3 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Gamma')), 4),
(520, 6, (SELECT id_equipe_coureur FROM equipe_coureur WHERE coureur = 4 AND equipe = (SELECT id_equipe FROM equipe WHERE nom = 'Equipe Delta')), 4);

-- Insertion des points
INSERT INTO points (valeur) VALUES 
(12), (10), (8), (6), (4), (2);
