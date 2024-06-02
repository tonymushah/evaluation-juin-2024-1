-- Your SQL goes here
create view v_coureur_point_etape as 
SELECT
	TE.COUREUR,
	SUM(TE.POINTS) AS POINTS,
	SUM(TE.TEMPS) AS TEMPS,
	Etape
FROM
	V_TEMPS_COUREUR_ETAPE_EQUIPE_COUREUR AS TE
GROUP BY te.coureur, etape;
