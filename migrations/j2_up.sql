CREATE OR REPLACE FUNCTION truncate_tables(username IN VARCHAR) RETURNS void AS $$
DECLARE
    statements CURSOR FOR
        SELECT tablename FROM pg_tables
        WHERE tableowner = username AND schemaname = 'public';
BEGIN
    FOR stmt IN statements LOOP
        EXECUTE 'TRUNCATE TABLE ' || quote_ident(stmt.tablename) || ' CASCADE;';
    END LOOP;
END;
$$ LANGUAGE plpgsql;

create table categorie(
    id_categorie UUID PRIMARY KEY default gen_random_uuid(),
    designation TEXT NOT NULL
);

create table coureur(
    numero_dosard SERIAL PRIMARY KEY,
    nom TEXT NOT NULL,
    genre INT NOT NULL,
    dtn DATE NOT NULL
);

create table equipe(
    id_equipe UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pseudo TEXT NOT NULL,
    mot_passe TEXT NOT NULL,
    nom TEXT NOT NULL
);

create table coureur_categorie(
    id_coureur_categorie UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    categorie UUID NOT NULL REFERENCES categorie(id_categorie),
    coureur INT NOT NULL REFERENCES coureur(numero_dosard)
);

create table equipe_coureur(
    id_equipe_coureur UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    coureur INT NOT NULL UNIQUE REFERENCES coureur(numero_dosard),
    equipe UUID NOT NULL REFERENCES equipe(id_equipe)
);

create table etape(
    rang SERIAL PRIMARY KEY,
    longueur DECIMAL NOT NULL,
    nom TEXT NOT NULL,
    nb_coureur_par_equipe INT NOT NULL,
    depart TIMESTAMP not NULL,
    finished TIMESTAMP 
);

create table temps_coureur(
    id_temps_coureur UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    temps INT,
    points INT,
    equipe_coureur UUID NOT NULL REFERENCES equipe_coureur(id_equipe_coureur),
    etape INT NOT NULL REFERENCES etape(rang)
);

create table points(
    rang SERIAL PRIMARY KEY,
    valeur INT NOT NULL
);

CREATE VIEW V_TEMPS_COUREUR_ETAPE AS
SELECT
	E.RANG AS ETAPE,
	E.LONGUEUR,
	E.NOM,
	E.NB_COUREUR_PAR_EQUIPE,
	TC.TEMPS,
	TC.POINTS,
	TC.EQUIPE_COUREUR,
	E.DEPART
FROM
	TEMPS_COUREUR AS TC
	JOIN ETAPE AS E ON TC.ETAPE = E.RANG;

-- Your SQL goes here
create VIEW v_temps_coureur_etape_equipe_coureur as 
SELECT
	TCE.ETAPE,
	TCE.LONGUEUR,
	TCE.NOM,
	TCE.NB_COUREUR_PAR_EQUIPE,
	TCE.TEMPS,
	TCE.POINTS,
	TCE.EQUIPE_COUREUR,
	TCE.DEPART,
	EC.COUREUR,
	EC.EQUIPE
FROM
	V_TEMPS_COUREUR_ETAPE AS TCE
	JOIN EQUIPE_COUREUR AS EC ON TCE.EQUIPE_COUREUR = EC.ID_EQUIPE_COUREUR;

create view v_equipe_coureur as 
SELECT
	EC.COUREUR,
	EC.EQUIPE,
	CR.NOM AS NOM_COUREUR,
	CR.GENRE,
	CR.DTN,
	E.PSEUDO AS PSEUDO_EQUIPE,
	E.NOM AS NOM_EQUIPE
FROM
	EQUIPE_COUREUR AS EC
	JOIN COUREUR AS CR ON EC.COUREUR = CR.NUMERO_DOSARD
	JOIN EQUIPE AS E ON EC.EQUIPE = E.ID_EQUIPE;

-- Your SQL goes here
create view v_coureur_point as 
SELECT
	TE.COUREUR,
	SUM(TE.POINTS) AS POINTS,
	SUM(TE.TEMPS) AS TEMPS
FROM
	V_TEMPS_COUREUR_ETAPE_EQUIPE_COUREUR AS TE
GROUP BY te.coureur;

create view v_equipe_point as 
SELECT
	TE.Equipe,
	SUM(TE.POINTS) AS POINTS,
	SUM(TE.TEMPS) AS TEMPS
FROM
	V_TEMPS_COUREUR_ETAPE_EQUIPE_COUREUR AS TE
GROUP BY te.equipe;

create view v_coureur_point_etape as 
SELECT
	TE.COUREUR,
	SUM(TE.POINTS) AS POINTS,
	SUM(TE.TEMPS) AS TEMPS,
	Etape
FROM
	V_TEMPS_COUREUR_ETAPE_EQUIPE_COUREUR AS TE
GROUP BY te.coureur, etape;

create view v_equipe_point_etape as 
 SELECT te.equipe,
    sum(te.points) AS points,
    sum(te.temps) AS temps,
	te.etape
   FROM v_temps_coureur_etape_equipe_coureur te
  GROUP BY te.equipe, te.etape;

