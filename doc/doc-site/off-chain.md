# Off-chain interfaces

## Registry events

Econia emits two types of registry events:

| Event                       | Event handle          | Field name                   |
|-----------------------------|-----------------------|------------------------------|
| [`MarketRegistrationEvent`] | [`Registry`]          | `market_registration_events` |
| [`RecognizedMarketEvent`]   | [`RecognizedMarkets`] | `recognized_market_events`   |

Event handles for registry events are created via the `@econia` package account, and are stored as fields in `key`-able resources stored under the `@econia` account.
Hence they can be easily queried via the Aptos node [events by event handle API].

## Market events

Econia emits two types of market events:

| Event          | Event handle  | Field name     |
|----------------|---------------|----------------|
| [`MakerEvent`] | [`OrderBook`] | `maker_events` |
| [`TakerEvent`] | [`OrderBook`] | `taker_events` |

Unlike registry event handles, market event handles are created via a resource account, since the `@econia` signature is not available each time a new [`OrderBook`] is permissionlessly initialized:
the resource account signing capability is stored under the `@econia` account in a [`SignerCapabilityStore`] (such that the account number can be easily queried off chain).
Additionally, each [`OrderBook`] is stored as a table entry in the [`OrderBooks`] map stored under the resource account, such that market events should be queried via the Aptos node [events by creation number API].

Since the resource account is initialized with an `aptos_framework::account::Account`, event stream creation numbers 0 and 1 are reserved for `coin_register_events` and `key_rotation_events` handles.
Hence, for 1-indexed market id `n`, maker events have creation number `2n` and taker events have creation number `2n + 1`:

| Market ID | [`MakerEvent`] handle creation number | [`TakerEvent`] handle creation number |
|-----------|---------------------------------------|---------------------------------------|
| 1         | 2                                     | 3                                     |
| 2         | 4                                     | 5                                     |
| 10        | 20                                    | 21                                    |
| `n`       | `2n`                                  | `2n + 1`                              |

## `move-to-ts` hooks

Econia is designed for use with [Hippo's `move-to-ts` tool], which auto-generates a TypeScript software development kit (SDK) from Move source code.
As such, Econia's Move source code contains assorted  `#[cmd]` attributes on public entry functions for SDK generation.
Additionally, [`index_orders_sdk()`] contains a `#[query]` attribute for off-chain [`OrderBook`] indexing.

The [`index_orders_sdk()`] function requires the signature of the `@simulation_account` address from [`Move.toml`], such that the function can not actually be executed as a public entry function during run time:
the function is only intended for use during transaction simulation mode (which does not require the private key of `@simulation_account`), via [Hippo's `move-to-ts` tool] `#[query]` interface.
Here, the simulation might fail for a large [`OrderBook`] due to transaction gas limits, and it may be necessary to set an artificially high gas limit so the simulation can process the entire data structure.

## Order book indexing

A suggested method for indexing an [`OrderBook`] in real time involves the [`index_orders_sdk()`] interface and event monitoring:

1. Run [`index_orders_sdk()`] to build up a local copy of an [`OrderBook`].
1. Read and apply all [`MakerEvent`] and [`TakerEvent`] emissions since the index operation.

Alternatively, all [`MakerEvent`] and [`TakerEvent`] emissions since the inception of the market can be applied consecutively.

<!---Alphabetized reference links-->

[events by creation number API]: https://fullnode.testnet.aptoslabs.com/v1/spec#/operations/get_events_by_creation_number
[events by event handle API]:    https://fullnode.testnet.aptoslabs.com/v1/spec#/operations/get_events_by_event_handle
[Hippo's `move-to-ts` tool]:     https://github.com/hippospace/move-to-ts
[`index_orders_sdk()`]:          ../../src/move/econia/doc/market.md#0xc0deb00c_market_index_orders_sdk
[`MakerEvent`]:                  ../../src/move/econia/doc/market.md#0xc0deb00c_market_MakerEvent
[`MarketRegistrationEvent`]:     ../../src/move/econia/doc/registry.md#0xc0deb00c_registry_MarketRegistrationEvent
[`Move.toml`]:                   ../../src/move/econia/Move.toml
[`OrderBook`]:                   ../../src/move/econia/doc/market.md#0xc0deb00c_market_OrderBook
[`OrderBooks`]:                  ../../src/move/econia/doc/market.md#0xc0deb00c_market_OrderBooks
[`RecognizedMarketEvent`]:       ../../src/move/econia/doc/registry.md#0xc0deb00c_registry_RecognizedMarketEvent
[`RecognizedMarkets`]:           ../../src/move/econia/doc/registry.md#0xc0deb00c_registry_RecognizedMarkets
[`Registry`]:                    ../../src/move/econia/doc/registry.md#0xc0deb00c_registry_Registry
[`SignerCapabilityStore`]:       ../../src/move/econia/doc/resource_account.md#0xc0deb00c_resource_account_SignerCapabilityStore
[`TakerEvent`]:                  ../../src/move/econia/doc/market.md#0xc0deb00c_market_TakerEvent