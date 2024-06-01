-- Your SQL goes here
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
    coureur INT NOT NULL REFERENCES coureur(numero_dosard),
    equipe UUID NOT NULL REFERENCES equipe(id_equipe)
);

create table etape(
    rang SERIAL PRIMARY KEY,
    longueur DECIMAL NOT NULL,
    nom TEXT NOT NULL,
    nb_coureur_par_equipe INT NOT NULL
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
	TC.EQUIPE_COUREUR
FROM
	TEMPS_COUREUR AS TC
	JOIN ETAPE AS E ON TC.ETAPE = E.RANG;