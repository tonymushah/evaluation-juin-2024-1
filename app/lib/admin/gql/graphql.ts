/* eslint-disable */
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
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
  TempsCoureur: { input: any; output: any; }
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
  Upload: { input: any; output: any; }
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

export type AdminMutations = {
  __typename?: 'AdminMutations';
  etape: EtapeMutation;
  import: ImportMutations;
  login: Scalars['String']['output'];
  logout: Scalars['Boolean']['output'];
  penalite: PenaliteMuations;
  resetDb: Scalars['Boolean']['output'];
};


export type AdminMutationsEtapeArgs = {
  id: Scalars['Int']['input'];
};


export type AdminMutationsLoginArgs = {
  password: Scalars['String']['input'];
};

export type AdminQueries = {
  __typename?: 'AdminQueries';
  equipe: EquipeQueries;
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

export type EquipeQueries = {
  __typename?: 'EquipeQueries';
  list: EquipeResults;
};


export type EquipeQueriesListArgs = {
  pagination: OffsetLimit;
};

export type EquipeResults = {
  __typename?: 'EquipeResults';
  data: Array<Equipe>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
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

export type EtapeMutation = {
  __typename?: 'EtapeMutation';
  addTime: TempCoureur;
};


export type EtapeMutationAddTimeArgs = {
  dosard: Scalars['Int']['input'];
  point?: InputMaybe<Scalars['Int']['input']>;
  temps: Scalars['TempsCoureur']['input'];
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

export type ImportMutations = {
  __typename?: 'ImportMutations';
  etapes: Array<Etape>;
  points: Array<Points>;
  resultats: Array<TempCoureur>;
};


export type ImportMutationsEtapesArgs = {
  file: Scalars['Upload']['input'];
};


export type ImportMutationsPointsArgs = {
  file: Scalars['Upload']['input'];
};


export type ImportMutationsResultatsArgs = {
  file: Scalars['Upload']['input'];
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

export type PenaliteInput = {
  equipe: Scalars['UUID']['input'];
  etape: Scalars['Int']['input'];
  idPenalite?: Scalars['UUID']['input'];
  valeur: Scalars['Int']['input'];
};

export type PenaliteMuations = {
  __typename?: 'PenaliteMuations';
  remove: Penalite;
  upsert: Penalite;
};


export type PenaliteMuationsRemoveArgs = {
  id: Scalars['UUID']['input'];
};


export type PenaliteMuationsUpsertArgs = {
  input: PenaliteInput;
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

export type Points = {
  __typename?: 'Points';
  rang: Scalars['Int']['output'];
  valeur: Scalars['Int']['output'];
};

export type TempCoureur = {
  __typename?: 'TempCoureur';
  equipeCoureur: Scalars['UUID']['output'];
  etape: Scalars['Int']['output'];
  idTempsCoureur: Scalars['UUID']['output'];
  points?: Maybe<Scalars['Int']['output']>;
  temps?: Maybe<Scalars['Int']['output']>;
};

export type ListAdminEtapeQueryVariables = Exact<{
  pagination: OffsetLimit;
}>;


export type ListAdminEtapeQuery = { __typename?: 'AdminQueries', etape: { __typename?: 'EtapeQueries', list: { __typename?: 'AdminEtapeResults', offset: number, limit: number, total: number, data: Array<{ __typename?: 'AdminEtape', depart: any, finished?: any | null, rang: number, nom: string, longueur: any, coureur: { __typename?: 'EtapeCoureurResults', total: number } }> } } };

export type GetAdminEtapeQueryVariables = Exact<{
  rang: Scalars['Int']['input'];
  courPage: OffsetLimit;
}>;


export type GetAdminEtapeQuery = { __typename?: 'AdminQueries', etape: { __typename?: 'EtapeQueries', unique: { __typename?: 'AdminEtape', depart: any, finished?: any | null, rang: number, nom: string, nbCoureurParEquipe: number, longueur: any, coureur: { __typename?: 'EtapeCoureurResults', limit: number, total: number, offset: number, data: Array<{ __typename?: 'EtapeCoureur', temps?: number | null, points?: number | null, equipeCoureur: any, coureur: { __typename?: 'Coureur', numeroDosard: number, nom: string }, equipe: { __typename?: 'Equipe', nom: string } }> } } } };

export type AddTimeToJoueurMutationVariables = Exact<{
  etape: Scalars['Int']['input'];
  dosard: Scalars['Int']['input'];
  temps: Scalars['TempsCoureur']['input'];
}>;


export type AddTimeToJoueurMutation = { __typename?: 'AdminMutations', etape: { __typename?: 'EtapeMutation', addTime: { __typename?: 'TempCoureur', temps?: number | null, etape: number, idTempsCoureur: any, points?: number | null } } };

export type ResetDbMutationVariables = Exact<{ [key: string]: never; }>;


export type ResetDbMutation = { __typename?: 'AdminMutations', resetDb: boolean };


export const ListAdminEtapeDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"listAdminEtape"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"pagination"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"OffsetLimit"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"etape"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"list"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"Variable","name":{"kind":"Name","value":"pagination"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"depart"}},{"kind":"Field","name":{"kind":"Name","value":"finished"}},{"kind":"Field","name":{"kind":"Name","value":"rang"}},{"kind":"Field","name":{"kind":"Name","value":"depart"}},{"kind":"Field","name":{"kind":"Name","value":"nom"}},{"kind":"Field","name":{"kind":"Name","value":"longueur"}},{"kind":"Field","name":{"kind":"Name","value":"coureur"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"ObjectValue","fields":[{"kind":"ObjectField","name":{"kind":"Name","value":"limit"},"value":{"kind":"IntValue","value":"1"}},{"kind":"ObjectField","name":{"kind":"Name","value":"offset"},"value":{"kind":"IntValue","value":"0"}}]}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"offset"}},{"kind":"Field","name":{"kind":"Name","value":"limit"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]} as unknown as DocumentNode<ListAdminEtapeQuery, ListAdminEtapeQueryVariables>;
export const GetAdminEtapeDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"getAdminEtape"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"rang"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"courPage"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"OffsetLimit"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"etape"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"unique"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"rang"},"value":{"kind":"Variable","name":{"kind":"Name","value":"rang"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"depart"}},{"kind":"Field","name":{"kind":"Name","value":"finished"}},{"kind":"Field","name":{"kind":"Name","value":"rang"}},{"kind":"Field","name":{"kind":"Name","value":"nom"}},{"kind":"Field","name":{"kind":"Name","value":"nbCoureurParEquipe"}},{"kind":"Field","name":{"kind":"Name","value":"longueur"}},{"kind":"Field","name":{"kind":"Name","value":"coureur"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"Variable","name":{"kind":"Name","value":"courPage"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"temps"}},{"kind":"Field","name":{"kind":"Name","value":"points"}},{"kind":"Field","name":{"kind":"Name","value":"coureur"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"numeroDosard"}},{"kind":"Field","name":{"kind":"Name","value":"nom"}}]}},{"kind":"Field","name":{"kind":"Name","value":"equipeCoureur"}},{"kind":"Field","name":{"kind":"Name","value":"equipe"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"nom"}}]}}]}},{"kind":"Field","name":{"kind":"Name","value":"limit"}},{"kind":"Field","name":{"kind":"Name","value":"total"}},{"kind":"Field","name":{"kind":"Name","value":"offset"}}]}}]}}]}}]}}]} as unknown as DocumentNode<GetAdminEtapeQuery, GetAdminEtapeQueryVariables>;
export const AddTimeToJoueurDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"addTimeToJoueur"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"etape"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"dosard"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"temps"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"TempsCoureur"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"etape"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"id"},"value":{"kind":"Variable","name":{"kind":"Name","value":"etape"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"addTime"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"dosard"},"value":{"kind":"Variable","name":{"kind":"Name","value":"dosard"}}},{"kind":"Argument","name":{"kind":"Name","value":"temps"},"value":{"kind":"Variable","name":{"kind":"Name","value":"temps"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"temps"}},{"kind":"Field","name":{"kind":"Name","value":"etape"}},{"kind":"Field","name":{"kind":"Name","value":"idTempsCoureur"}},{"kind":"Field","name":{"kind":"Name","value":"points"}}]}}]}}]}}]} as unknown as DocumentNode<AddTimeToJoueurMutation, AddTimeToJoueurMutationVariables>;
export const ResetDbDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"resetDB"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"resetDb"}}]}}]} as unknown as DocumentNode<ResetDbMutation, ResetDbMutationVariables>;