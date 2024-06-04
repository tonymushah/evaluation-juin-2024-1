/* eslint-disable */
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  BigDecimal: { input: any; output: any; }
  /**
   * ISO 8601 calendar date without timezone.
   * Format: %Y-%m-%d
   *
   * # Examples
   *
   * * `1994-11-13`
   * * `2000-02-24`
   */
  Date: { input: any; output: any; }
  /**
   * A local datetime without timezone offset.
   *
   * The input/output is a string in ISO 8601 format without timezone, including
   * subseconds. E.g. "2022-01-12T07:30:19.12345".
   */
  LocalDateTime: { input: any; output: any; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: any; output: any; }
};

export type AdminEtape = {
  __typename?: 'AdminEtape';
  coureur: EtapeCoureurResults;
  depart: Scalars['LocalDateTime']['output'];
  finished?: Maybe<Scalars['LocalDateTime']['output']>;
  longueur: Scalars['BigDecimal']['output'];
  nbCoureurParEquipe: Scalars['Int']['output'];
  nom: Scalars['String']['output'];
  rang: Scalars['Int']['output'];
};


export type AdminEtapeCoureurArgs = {
  pagination: OffsetLimit;
};

export type AdminEtapeResults = {
  __typename?: 'AdminEtapeResults';
  data: Array<AdminEtape>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type AdminQueries = {
  __typename?: 'AdminQueries';
  etape: EtapeQueries;
  hello: Scalars['String']['output'];
  penalite: PenalitesQueries;
};

export type Coureur = {
  __typename?: 'Coureur';
  dtn: Scalars['Date']['output'];
  genre: Scalars['Int']['output'];
  nom: Scalars['String']['output'];
  numeroDosard: Scalars['Int']['output'];
};

export type Equipe = {
  __typename?: 'Equipe';
  idEquipe: Scalars['UUID']['output'];
  nom: Scalars['String']['output'];
  pseudo: Scalars['String']['output'];
};

export type Etape = {
  __typename?: 'Etape';
  depart: Scalars['LocalDateTime']['output'];
  finished?: Maybe<Scalars['LocalDateTime']['output']>;
  longueur: Scalars['BigDecimal']['output'];
  nbCoureurParEquipe: Scalars['Int']['output'];
  nom: Scalars['String']['output'];
  rang: Scalars['Int']['output'];
};

export type EtapeCoureur = {
  __typename?: 'EtapeCoureur';
  coureur: Coureur;
  equipe: Equipe;
  equipeCoureur: Scalars['UUID']['output'];
  points?: Maybe<Scalars['Int']['output']>;
  temps?: Maybe<Scalars['Int']['output']>;
};

export type EtapeCoureurResults = {
  __typename?: 'EtapeCoureurResults';
  data: Array<EtapeCoureur>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type EtapeQueries = {
  __typename?: 'EtapeQueries';
  list: AdminEtapeResults;
  unique: AdminEtape;
};


export type EtapeQueriesListArgs = {
  pagination: OffsetLimit;
};


export type EtapeQueriesUniqueArgs = {
  rang: Scalars['Int']['input'];
};

export type OffsetLimit = {
  limit: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
};

export type Penalite = {
  __typename?: 'Penalite';
  equipe: Scalars['UUID']['output'];
  equipeData: Equipe;
  etape: Scalars['Int']['output'];
  etapeData: Etape;
  idPenalite: Scalars['UUID']['output'];
  valeur: Scalars['Int']['output'];
};

export type PenalitesQueries = {
  __typename?: 'PenalitesQueries';
  list: PenalitesResults;
};


export type PenalitesQueriesListArgs = {
  pagination?: OffsetLimit;
};

export type PenalitesResults = {
  __typename?: 'PenalitesResults';
  data: Array<Penalite>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};
