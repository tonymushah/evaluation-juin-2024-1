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

export type Categorie = {
  __typename?: 'Categorie';
  designation: Scalars['String']['output'];
  idCategorie: Scalars['UUID']['output'];
};

export type CategorieResults = {
  __typename?: 'CategorieResults';
  data: Array<Categorie>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type ClassementQueries = {
  __typename?: 'ClassementQueries';
  parCategorie: CoureurPointResults;
  parCoureur: CoureurPointResults;
  parEquipe: EquipePointResults;
};


export type ClassementQueriesParCategorieArgs = {
  id: Scalars['UUID']['input'];
  ordre?: InputMaybe<GraphQlOrdering>;
  pagination?: OffsetLimit;
};


export type ClassementQueriesParCoureurArgs = {
  ordre?: InputMaybe<GraphQlOrdering>;
  pagination?: OffsetLimit;
};


export type ClassementQueriesParEquipeArgs = {
  ordre?: InputMaybe<GraphQlOrdering>;
  pagination?: OffsetLimit;
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

export type EquipePoint = {
  __typename?: 'EquipePoint';
  equipe: Scalars['UUID']['output'];
  nom: Scalars['String']['output'];
  points: Scalars['Int']['output'];
  temps: Scalars['Int']['output'];
};

export type EquipePointResults = {
  __typename?: 'EquipePointResults';
  data: Array<EquipePoint>;
  limit: Scalars['Int']['output'];
  offset: Scalars['Int']['output'];
  total: Scalars['Int']['output'];
};

export type GlobalQueries = {
  __typename?: 'GlobalQueries';
  categories: CategorieResults;
  classements: ClassementQueries;
  getCoureur: VequipeCoureur;
  hello: Scalars['String']['output'];
};


export type GlobalQueriesCategoriesArgs = {
  pagination: OffsetLimit;
};


export type GlobalQueriesGetCoureurArgs = {
  coureur: Scalars['Int']['input'];
};

export enum GraphQlOrdering {
  Ascending = 'ASCENDING',
  Descending = 'DESCENDING'
}

export type OffsetLimit = {
  limit: Scalars['Int']['input'];
  offset: Scalars['Int']['input'];
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

export type ClassementEquipeGeneraleQueryVariables = Exact<{
  ordre?: InputMaybe<GraphQlOrdering>;
  pagination?: InputMaybe<OffsetLimit>;
}>;


export type ClassementEquipeGeneraleQuery = { __typename?: 'GlobalQueries', classements: { __typename?: 'ClassementQueries', parEquipe: { __typename?: 'EquipePointResults', limit: number, offset: number, total: number, data: Array<{ __typename?: 'EquipePoint', equipe: any, nom: string, temps: number, points: number }> } } };

export type ClassementGeneraleQueryVariables = Exact<{
  ordre?: InputMaybe<GraphQlOrdering>;
  pagination?: InputMaybe<OffsetLimit>;
}>;


export type ClassementGeneraleQuery = { __typename?: 'GlobalQueries', classements: { __typename?: 'ClassementQueries', parCoureur: { __typename?: 'CoureurPointResults', limit: number, offset: number, total: number, data: Array<{ __typename?: 'CoueurPoint', coureur: number, temps: number, points: number }> } } };


export const ClassementEquipeGeneraleDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"classementEquipeGenerale"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"ordre"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"GraphQLOrdering"}},"defaultValue":{"kind":"EnumValue","value":"DESCENDING"}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"pagination"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"OffsetLimit"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"classements"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"parEquipe"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"ordre"},"value":{"kind":"Variable","name":{"kind":"Name","value":"ordre"}}},{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"Variable","name":{"kind":"Name","value":"pagination"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"equipe"}},{"kind":"Field","name":{"kind":"Name","value":"nom"}},{"kind":"Field","name":{"kind":"Name","value":"temps"}},{"kind":"Field","name":{"kind":"Name","value":"points"}}]}},{"kind":"Field","name":{"kind":"Name","value":"limit"}},{"kind":"Field","name":{"kind":"Name","value":"offset"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]} as unknown as DocumentNode<ClassementEquipeGeneraleQuery, ClassementEquipeGeneraleQueryVariables>;
export const ClassementGeneraleDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"classementGenerale"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"ordre"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"GraphQLOrdering"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"pagination"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"OffsetLimit"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"classements"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"parCoureur"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"ordre"},"value":{"kind":"Variable","name":{"kind":"Name","value":"ordre"}}},{"kind":"Argument","name":{"kind":"Name","value":"pagination"},"value":{"kind":"Variable","name":{"kind":"Name","value":"pagination"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"data"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"coureur"}},{"kind":"Field","name":{"kind":"Name","value":"temps"}},{"kind":"Field","name":{"kind":"Name","value":"points"}}]}},{"kind":"Field","name":{"kind":"Name","value":"limit"}},{"kind":"Field","name":{"kind":"Name","value":"offset"}},{"kind":"Field","name":{"kind":"Name","value":"total"}}]}}]}}]}}]} as unknown as DocumentNode<ClassementGeneraleQuery, ClassementGeneraleQueryVariables>;