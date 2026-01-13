---
description: The page is under development
---

# Integration

## Getting Started

This section describes the minimal steps required to integrate Bee Engine into your application and start client-side NACKL mining.

### Step 1. Connecting the Bee Engine Mining Library

To use Bee Engine, you need a WebAssembly module: `bee_engine_miner_bg.wasm`

This file contains the client mining engine and must be accessible to your application via HTTP.

#### Hosting the WASM File _**(in progress)**_

For web applications (React / Vite):

1. Download the archive [bee\_engine.zip](https://binaries.gosh.sh/bee_engine/bee_engine.zip) and extract it into the folder. The archive contains the `bee_engine_miner_bg.wasm` module and its bindings.
2. Ensure the file is accessible via URL, for example: `/bee_engine_miner_bg.wasm`

#### Integration Example (React + Vite)

Below is a minimal example of initializing and controlling the Bee Engine Miner in a React application.

**App.tsx**

```tsx
import viteLogo from "/vite.svg";
import reactLogo from "./assets/react.svg";
import "./App.css";

import { init, Miner } from "/path/to/bee_engine/folder";
import { useState } from "react";

async function initMiner() {
await init({ module_or_path: "/bee_engine_miner_bg.wasm" });
return await Miner.new(
 ["<blockchain_endpoint>"],
 "<app_dapp_id>",
 "<miner_address>",
 "<mining_public_key>",
 "<mining_secret_key>",
);
}

function minerCallback(message: object) {
// Here you can read events from the miner and process them
console.log(`[MINER_CALLBACK]: ${message}`);
}

function App() {
const [miner, setMiner] = useState<Miner>();

return (
 <>
   <div>
     <a href="https://vite.dev" target="_blank" rel="noreferrer">
       <img src={viteLogo} className="logo" alt="Vite logo" />
     </a>
     <a href="https://react.dev" target="_blank" rel="noreferrer">
       <img src={reactLogo} className="logo react" alt="React logo" />
     </a>
   </div>
   <h1>Vite + React</h1>

   <div style={{ display: "flex", gap: "16px" }}>
     <button
       type="button"
       onClick={async () => {
         miner?.free();

         const instance = await initMiner();
         setMiner(instance);
       }}
     >
       Init miner
     </button>
     <button type="button" onClick={() => miner?.start(15000, minerCallback)}>
       Start miner
     </button>
     <button type="button" onClick={() => miner?.add_tap(1, 1)}>
       Add tap
     </button>
     <button type="button" onClick={() => miner?.stop()}>
       Stop miner
     </button>
   </div>
 </>
);
}
export default App;
```

### Step 2. Application Registration _**(in progress)**_

Before users can start mining, your application must be registered.

1. Register your application in the **Acki Nacki App Store** (internal registry)
2. Obtain the application address in the system `app_dapp_id`

This value will be used when initializing the Bee Engine miner and will link the mining results to your application.

### Step 3. User Authorization _**(in progress)**_

Each user of your application must:

* Authorize via [**Acki Nacki Wallet**](https://ackinacki.com/wallet)
* Select your application from the list
* Confirm access

After authorization:

* Bee Engine automatically receives the user’s mining keys
* Keys are used to sign and submit mining results

#### Limitations

* One wallet can be connected to **no more than 100 applications**
* Exceeding this limit will cause new connections to be rejected

### Step 4. Working with the Bee Engine API

Below are the core `bee_engine_miner` methods used to control mining.

#### `can_start() -> bool`

Checks whether mining can be started.

Returns `true` if:

* the Bee Engine Miner is not running
* there is no active mining process
* there are available seeds to work with

⚠️ If you call `start()` without checking and mining is not possible, an error will be thrown.

#### `start(duration_ms: number, callback: (event: object) -> void) -> void`

Starts the mining process for a specified duration.

* `duration_ms` — session duration in milliseconds
* `callback` — function receiving miner events

After starting:

* the Bee Engine Miner begins hashing with reduced difficulty
* events (status, progress, errors) are delivered via callback

#### `add_tap(x: number, y: number) -> void`

Adds a user action (tap) to the Merkle Tree.

Features:

* this hash is computed with increased difficulty
* used to bind user activity to mining
* coordinates `(x, y)` are defined by the application

#### `stop() -> void`

Forcefully stops the Bee Engine Miner.

When called:

* mining is terminated
* results are sent to the contract for validation

If not called, the Bee Engine Miner:

* stops automatically after `duration_ms`
* submits results on its own

#### `get_reward() -> void`

Collects available rewards from previous mining sessions.

Recommendations:

* no need to call more than once per epoch (\~1000 blocks)
* rewards are collected automatically when submitting data to the contract

Use this method if:

* the application has just launched
* you need to explicitly synchronize the user’s balance

#### &#x20;`polling()`

A special function that:

* polls the mining contract
* waits for the key pair requested by Bee Engine via AN Wallet
* is used to synchronize state during authorization

### What’s Next

After basic integration, you can:

* Start and stop the mining process
* bind `add_tap` to user actions
* use miner events for UI / telemetry
* integrate reputation and economics into your application

