-- Your SQL goes here
create view v_equipe_point as 
SELECT
	TE.Equipe,
	SUM(TE.POINTS) AS POINTS,
	SUM(TE.TEMPS) AS TEMPS
FROM
	V_TEMPS_COUREUR_ETAPE_EQUIPE_COUREUR AS TE
GROUP BY te.equipe;