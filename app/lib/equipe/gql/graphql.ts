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

export type CoueurPoint = {
  __typename?: 'CoueurPoint';
  coureur: Scalars['Int']['output'];
  points: Scalars['Int']['output'];
  temps: Scalars['Int']['output'];
};

export type CoureurPointResults = {
  __typename?: 'CoureurPointResults';
  data: Array<CoueurPoint>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type Equipe = {
  __typename?: 'Equipe';
  idEquipe: Scalars['UUID']['output'];
  nom: Scalars['String']['output'];
  pseudo: Scalars['String']['output'];
};

export type EquipeMutations = {
  __typename?: 'EquipeMutations';
  ajouterJoueurToEtape: TempCoureur;
  login: Scalars['String']['output'];
  removeJoueurToEtape: TempCoureur;
};


export type EquipeMutationsAjouterJoueurToEtapeArgs = {
  etape: Scalars['Int']['input'];
  joueur: Scalars['Int']['input'];
};


export type EquipeMutationsLoginArgs = {
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};


export type EquipeMutationsRemoveJoueurToEtapeArgs = {
  etape: Scalars['Int']['input'];
  joueur: Scalars['Int']['input'];
};

export type EquipeQueries = {
  __typename?: 'EquipeQueries';
  classementParCategorie: CoureurPointResults;
  coureur: VequipeCoureur;
  current: Equipe;
  etape: EtapeQueries;
  hello: Scalars['String']['output'];
  listCoureur: VEquipeCoureurResults;
};


export type EquipeQueriesClassementParCategorieArgs = {
  categorie: Scalars['UUID']['input'];
  ordre?: InputMaybe<GraphQlOrdering>;
  pagination?: OffsetLimit;
};


export type EquipeQueriesCoureurArgs = {
  dosard: Scalars['Int']['input'];
};


export type EquipeQueriesListCoureurArgs = {
  pagination: OffsetLimit;
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

export type EtapeQueries = {
  __typename?: 'EtapeQueries';
  joueurs: Array<VequipeCoureur>;
  list: EtapeResults;
  unique: Etape;
};


export type EtapeQueriesJoueursArgs = {
  etape: Scalars['Int']['input'];
};


export type EtapeQueriesListArgs = {
  pagination: OffsetLimit;
};


export type EtapeQueriesUniqueArgs = {
  rang: Scalars['Int']['input'];
};

export type EtapeResults = {
  __typename?: 'EtapeResults';
  data: Array<Etape>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export enum GraphQlOrdering {
  Ascending = 'ASCENDING',
  Descending = 'DESCENDING'
}

export type OffsetLimit = {
  limit: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
};

export type TempCoureur = {
  __typename?: 'TempCoureur';
  equipeCoureur: Scalars['UUID']['output'];
  etape: Scalars['Int']['output'];
  idTempsCoureur: Scalars['UUID']['output'];
  points?: Maybe<Scalars['Int']['output']>;
  temps?: Maybe<Scalars['Int']['output']>;
};

export type VEquipeCoureurResults = {
  __typename?: 'VEquipeCoureurResults';
  data: Array<VequipeCoureur>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type VequipeCoureur = {
  __typename?: 'VequipeCoureur';
  coureur: Scalars['Int']['output'];
  dtn: Scalars['Date']['output'];
  equipe: Scalars['UUID']['output'];
  genre: Scalars['Int']['output'];
  idEquipeCoureur: Scalars['UUID']['output'];
  nomCoureur: Scalars['String']['output'];
  nomEquipe: Scalars['String']['output'];
  points?: Maybe<Scalars['Int']['output']>;
};

export type EtapeCoureursQueryVariables = Exact<{
  etape: Scalars['Int']['input'];
}>;


export type EtapeCoureursQuery = { __typename?: 'EquipeQueries', etape: { __typename?: 'EtapeQueries', joueurs: Array<{ __typename?: 'VequipeCoureur', coureur: number, nomCoureur: string, points?: number | null }> } };

export type ListEtapesQueryVariables = Exact<{
  page: OffsetLimit;
}>;


export type ListEtapesQuery = { __typename?: 'EquipeQueries', etape: { __typename?: 'EtapeQueries', list: { __typename?: 'EtapeResults', limit: number, offset: number, total: number, data: Array<{ __typename?: 'Etape', rang: number, longueur: any, nom: string, depart: any, finished?: any | null }> } } };

export type CurrentEquipeQueryVariables = Exact<{ [key: string]: never; }>;


export type CurrentEquipeQuery = { __typename?: 'EquipeQueries', current: { __typename?: 'Equipe', pseudo: string, nom: string, idEquipe: any } };

export type EquipeLoginMutationVariables = Exact<{
  user: Scalars['String']['input'];
  password: Scalars['String']['input'];
}>;


export type EquipeLoginMutation = { __typename?: 'EquipeMutations', login: string };


export const EtapeCoureursDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"etapeCoureurs"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"etape"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"Int"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"etape"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"joueurs"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"etape"},"value":{"kind":"Variable","name":{"kind":"Name","value":"etape"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"coureur"}},{"kind":"Field","name":{"kind":"Name","value":"nomCoureur"}},{"kind":"Field","name":{"kind":"Name","value":"points"}}]}}]}}]}}]} as unknown as DocumentNode<EtapeCoureursQuery, EtapeCoureursQueryVariables>;
export const ListEtapesDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"listEtapes"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"page"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"OffsetLimit"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"etape"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"list"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"Variable","name":{"kind":"Name","value":"page"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"rang"}},{"kind":"Field","name":{"kind":"Name","value":"longueur"}},{"kind":"Field","name":{"kind":"Name","value":"nom"}},{"kind":"Field","name":{"kind":"Name","value":"depart"}},{"kind":"Field","name":{"kind":"Name","value":"finished"}}]}},{"kind":"Field","name":{"kind":"Name","value":"limit"}},{"kind":"Field","name":{"kind":"Name","value":"offset"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]} as unknown as DocumentNode<ListEtapesQuery, ListEtapesQueryVariables>;
export const CurrentEquipeDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"currentEquipe"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"current"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"pseudo"}},{"kind":"Field","name":{"kind":"Name","value":"nom"}},{"kind":"Field","name":{"kind":"Name","value":"idEquipe"}}]}}]}}]} as unknown as DocumentNode<CurrentEquipeQuery, CurrentEquipeQueryVariables>;
export const EquipeLoginDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"equipeLogin"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"user"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"password"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"login"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"username"},"value":{"kind":"Variable","name":{"kind":"Name","value":"user"}}},{"kind":"Argument","name":{"kind":"Name","value":"password"},"value":{"kind":"Variable","name":{"kind":"Name","value":"password"}}}]}]}}]} as unknown as DocumentNode<EquipeLoginMutation, EquipeLoginMutationVariables>;