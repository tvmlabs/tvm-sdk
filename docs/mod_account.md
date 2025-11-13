# Module account

Provides information about account.


## Functions
[get_account](mod\_account.md#get_account)

## Types
[AccountErrorCode](mod\_account.md#accounterrorcode)

[ParamsOfGetAccount](mod\_account.md#paramsofgetaccount)

[ResultOfGetAccount](mod\_account.md#resultofgetaccount)


# Functions
## get_account

```ts
type ParamsOfGetAccount = {
    address: string
}

type ResultOfGetAccount = {
    boc: string,
    dapp_id?: string,
    state_timestamp?: bigint
}

function get_account(
    params: ParamsOfGetAccount,
): Promise<ResultOfGetAccount>;

function get_account_sync(
    params: ParamsOfGetAccount,
): ResultOfGetAccount;
```
NOTE: Sync version is available only for `lib-node` binding.
### Parameters
- `address`: _string_


### Result

- `boc`: _string_
- `dapp_id`?: _string_
- `state_timestamp`?: _bigint_


# Types
## AccountErrorCode
```ts
enum AccountErrorCode {
    NotImplemented = 1,
    InvalidHex = 2,
    InvalidBase64 = 3,
    InvalidAddress = 4,
    CallbackParamsCantBeConvertedToJson = 5,
    WebsocketConnectError = 6,
    WebsocketReceiveError = 7,
    WebsocketSendError = 8,
    HttpClientCreateError = 9,
    HttpRequestCreateError = 10,
    HttpRequestSendError = 11,
    HttpRequestParseError = 12,
    CallbackNotRegistered = 13,
    NetModuleNotInit = 14,
    InvalidConfig = 15,
    CannotCreateRuntime = 16,
    InvalidContextHandle = 17,
    CannotSerializeResult = 18,
    CannotSerializeError = 19,
    CannotConvertJsValueToJson = 20,
    CannotReceiveSpawnedResult = 21,
    SetTimerError = 22,
    InvalidParams = 23,
    ContractsAddressConversionFailed = 24,
    UnknownFunction = 25,
    AppRequestError = 26,
    NoSuchRequest = 27,
    CanNotSendRequestResult = 28,
    CanNotReceiveRequestResult = 29,
    CanNotParseRequestResult = 30,
    UnexpectedCallbackResponse = 31,
    CanNotParseNumber = 32,
    InternalError = 33,
    InvalidHandle = 34,
    LocalStorageError = 35,
    InvalidData = 36
}
```
One of the following value:

- `NotImplemented = 1`
- `InvalidHex = 2`
- `InvalidBase64 = 3`
- `InvalidAddress = 4`
- `CallbackParamsCantBeConvertedToJson = 5`
- `WebsocketConnectError = 6`
- `WebsocketReceiveError = 7`
- `WebsocketSendError = 8`
- `HttpClientCreateError = 9`
- `HttpRequestCreateError = 10`
- `HttpRequestSendError = 11`
- `HttpRequestParseError = 12`
- `CallbackNotRegistered = 13`
- `NetModuleNotInit = 14`
- `InvalidConfig = 15`
- `CannotCreateRuntime = 16`
- `InvalidContextHandle = 17`
- `CannotSerializeResult = 18`
- `CannotSerializeError = 19`
- `CannotConvertJsValueToJson = 20`
- `CannotReceiveSpawnedResult = 21`
- `SetTimerError = 22`
- `InvalidParams = 23`
- `ContractsAddressConversionFailed = 24`
- `UnknownFunction = 25`
- `AppRequestError = 26`
- `NoSuchRequest = 27`
- `CanNotSendRequestResult = 28`
- `CanNotReceiveRequestResult = 29`
- `CanNotParseRequestResult = 30`
- `UnexpectedCallbackResponse = 31`
- `CanNotParseNumber = 32`
- `InternalError = 33`
- `InvalidHandle = 34`
- `LocalStorageError = 35`
- `InvalidData = 36`


## ParamsOfGetAccount
```ts
type ParamsOfGetAccount = {
    address: string
}
```
- `address`: _string_


## ResultOfGetAccount
```ts
type ResultOfGetAccount = {
    boc: string,
    dapp_id?: string,
    state_timestamp?: bigint
}
```
- `boc`: _string_
- `dapp_id`?: _string_
- `state_timestamp`?: _bigint_


