-- Your SQL goes here
CREATE VIEW V_CLASSEMENT_ETAPE_CATEGORIE AS
SELECT
	TCEEC.COUREUR,
	TCEEC.CATEGORIE,
	TCEEC.ETAPE,
	SUM(TCEEC.POINTS) AS POINTS,
	SUM(TCEEC.TEMPS) AS TEMPS
FROM
	V_TEMPS_COUREUR_ETAPE_EQUIPE_COUREUR_CATEGORIE AS TCEEC
GROUP BY
	TCEEC.COUREUR,
	TCEEC.CATEGORIE,
	TCEEC.ETAPE;