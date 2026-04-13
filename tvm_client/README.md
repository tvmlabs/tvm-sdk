# TON SDK Core Library 1.0


# Dynamic Library Interface


- `tc_create_context` – creates core context
- `tc_request_context_async` – performs async request to core function
- `tc_destroy_context` – closes core context
    
    
# Async JSON Interface (API)

   
- `tc_request_context_async (request_id, method, params_json, on_response)` – sends 
    async request to core
- `on_response (request_id, result_json, error_json)` – receives async response 
    from core
    

# Async App Callbacks (new)


- `context.register_callback` – registers async app defined callback into core  
- `on_callback` – receives callback invocation from core 
- `context.subscribe` – example of callback usage  
- `context.unregister_callback` – unregisters callback


# Core Modules (reorganized)


- `client` – core interoperation functions  
- `crypto` – cryptography
- `abi` – message encoding / decoding according to ABI specs
- `boc` – BOC serialization / manipulation / block structures parsing
- `tvm` – local TVM runtime utilization (get methods, local run, etc.)
- `net` – blockchain interaction functions


# BOC Module


- `parse_account`
- `parse_message`
- `parse_transaction`
- `parse_block`


# ABI Module


- `encode_message` – instead of a lot of encode functions
- `decode_message` – instead of a lot of decode functions  

    
# Net Module


- `process_message` – observable single function with callback support
    instead of a lot of runs deploys, waits for transactions etc.
- `discover_bm` – forces re-discovery of the active Block Manager (BM) endpoint
    by broadcasting readiness checks to all configured endpoints


## Block Manager Discovery and Failover

When multiple endpoints are configured, the client automatically discovers the
best Block Manager by broadcasting `GET /v2/readiness` to all endpoints on
first message send. The first endpoint to respond with HTTP 200 becomes the
active BM.

- **BM failover**: if the active BM becomes unreachable during message delivery,
  the client re-runs discovery to find a new working BM.
- **BP fallback proxy**: when `fallback_proxy_mode` is enabled in `NetworkConfig`
  and a Block Producer (BP) is unreachable from the client (e.g. due to firewall),
  the client routes all subsequent messages through the BM, which proxies them
  to the BP. This mode is sticky and stays active until client restart.

### Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `fallback_proxy_mode` | `bool` | `false` | Enable sticky fallback proxy through BM when BP is unreachable |
| `bm_readiness_timeout` | `u32` | `5000` | Timeout (ms) for each BM readiness check during discovery |


## REST API Endpoint Construction

Endpoint addresses from `NetworkConfig.endpoints` and from `producers[]`
redirect responses are converted to REST API URLs by `construct_rest_api_endpoint`.

**Rules:**

1. **Scheme selection** — if the address has no scheme (`://`), the scheme is
   chosen automatically: `https` when the first configured endpoint uses HTTPS,
   `http` otherwise. An explicit scheme in the address is always preserved.
2. **Standard ports** — addresses without an explicit port use the standard
   port for their scheme (80 for HTTP, 443 for HTTPS).
3. **Explicit ports** — a port specified in the address is kept as-is for HTTP.
   For HTTPS the port is always reset to 443 (explicit ports are stripped).
4. **Path** — the path is set to `/v2/` regardless of the original path.

**Examples:**

| Input address | Scheme context | Result URL |
|---|---|---|
| `bk.example.ackinacki.org` | HTTPS | `https://bk.example.ackinacki.org/v2/` |
| `bk.example.ackinacki.org` | HTTP | `http://bk.example.ackinacki.org/v2/` |
| `bk.example.ackinacki.org:9090` | HTTP | `http://bk.example.ackinacki.org:9090/v2/` |
| `bk.example.ackinacki.org:9090` | HTTPS | `https://bk.example.ackinacki.org/v2/` (port stripped) |
| `http://10.0.0.1:8080` | any | `http://10.0.0.1:8080/v2/` |
| `https://10.0.0.1:9090` | any | `https://10.0.0.1/v2/` (port stripped) |


## HTTP Retry for Message Delivery

`query_http()` — the transport layer used by `send_message` — automatically
retries requests on transient failures before returning an error to the caller.

**Retryable conditions:**

- Transport errors — connection refused, connection reset, timeout
- HTTP status codes — 429, 500, 502, 503, 504

**Non-retryable (returned immediately):**

- HTTP 4xx (except 429) — client error, retry won't help
- HTTP 2xx — success
- Response body parse error — server responded, but body is unreadable
- Request build error — problem on client side

**Parameters:**

| Parameter | Value | Description |
|---|---|---|
| Max attempts | 3 | 1 original + 2 retries |
| Initial delay | 200 ms | Delay before the first retry |
| Backoff strategy | exponential | `delay = 200ms * 2^attempt` (200 ms, 400 ms) |

Each retry is logged at WARN level with the endpoint URL, attempt number,
reason (error or HTTP status), and delay before the next attempt.

This retry operates at the transport level and is transparent to the caller.
Business-level retries in `send_message` (WRONG_PRODUCER, THREAD_MISMATCH,
TOKEN_EXPIRED) work on top of it independently.


# Internal Code Refactorings


- `modularity` – each module placed inside own folder
- `similarity` – each API function has parameters represented as a structure 
    and a result represented as a structure
- `encodings` – each var len byte buffer encoded with `base64` 
    and each fixed len byte buffer encoded with `hex`
- `errors` – each module has own errors definitions (instead of huge common file)
- `tests` – each module has own test suite (instead of huge common test suite)
- `self-documented` – each API function and structure has self documented 
    derived at runtime documentation
- `rust-direct` – all underlying API functions are accessible directly 
    from rust applications




