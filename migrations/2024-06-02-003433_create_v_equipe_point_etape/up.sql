-- Your SQL goes here
create view v_equipe_point_etape as 
 SELECT te.equipe,
    sum(te.points) AS points,
    sum(te.temps) AS temps,
	te.etape
   FROM v_temps_coureur_etape_equipe_coureur te
  GROUP BY te.equipe, te.etape;