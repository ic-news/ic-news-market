import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Listing {
  'id' : number,
  'self_reported_circulating_supply' : number,
  'num_market_pairs' : number,
  'name' : string,
  'slug' : string,
  'tags' : Array<string>,
  'quote' : Quote,
  'last_updated' : string,
  'self_reported_market_cap' : number,
  'cmc_rank' : number,
  'platform' : [] | [Platform],
  'circulating_supply' : number,
  'date_added' : string,
  'max_supply' : number,
  'total_supply' : number,
  'symbol' : string,
  'infinite_supply' : boolean,
}
export interface Platform {
  'id' : number,
  'name' : string,
  'slug' : string,
  'token_address' : string,
  'symbol' : string,
}
export interface Quote {
  'market_cap' : number,
  'percent_change_1h' : number,
  'percent_change_7d' : number,
  'volume_change_24h' : number,
  'volume_24h' : number,
  'last_updated' : string,
  'fully_diluted_market_cap' : number,
  'percent_change_24h' : number,
  'percent_change_30d' : number,
  'percent_change_60d' : number,
  'percent_change_90d' : number,
  'price' : number,
  'market_cap_dominance' : number,
}
export interface _SERVICE {
  'add_manager' : ActorMethod<
    [Principal],
    { 'Ok' : null } |
      { 'Err' : string }
  >,
  'delete_listing' : ActorMethod<
    [string],
    { 'Ok' : null } |
      { 'Err' : string }
  >,
  'get_listing' : ActorMethod<[string], [] | [Listing]>,
  'get_listings' : ActorMethod<[[] | [bigint], [] | [bigint]], Array<Listing>>,
  'list_managers' : ActorMethod<
    [],
    { 'Ok' : Array<Principal> } |
      { 'Err' : string }
  >,
  'remove_manager' : ActorMethod<
    [Principal],
    { 'Ok' : null } |
      { 'Err' : string }
  >,
  'upsert_listing' : ActorMethod<
    [Listing],
    { 'Ok' : null } |
      { 'Err' : string }
  >,
  'upsert_listings' : ActorMethod<
    [Array<Listing>],
    { 'Ok' : null } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
