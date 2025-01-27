# Add SDK to your App

[Node.js](add_sdk_to_your_app.md#nodejs) | [Web](add_sdk_to_your_app.md#web)

## Node.js

> Our library is fully-annotated with `.d.ts` files so we recommend to write your applications in Typescript.

Let's start with a clean npm project.

```
mkdir hello
cd hello
npm init -y
```

Now lets install core package and bridge package for Node.js

```
npm i --save @tvmsdk/core
npm i --save @tvmsdk/lib-node
```

You must initialize the library before the first use. The best place to do it is an initialization code of your application.

You need to attach the chosen binary module to the `TvmClient` class. Create `index.js` file and add this code:

```
const {TonClient} = require("@eversdk/core");
const {libNode} = require("@eversdk/lib-node");

// Application initialization
TonClient.useBinaryLibrary(libNode)
```

That's it! Now you are ready to create and[ configure TvmClient object!](../configuration/endpoint-configuration.md)

## Web

> Our library is fully-annotated with `.d.ts` files so we recommend to write your applications in Typescript.

Let's start with a clean project.

```
mkdir hello
cd hello
npm init -y
```

**Installation**

Now lets install core package and bridge package for Web

```
npm i --save @tvmsdk/core
npm i --save @tvmsdk/lib-web
```

**Important!** Each time you run `npm install` the new version of the `tvmsdk.wasm` and `index.js` is downloaded. So you have to always update the `tvmsdk.wasm` inside your web package before publishing (starting local web server, creating web bundle etc.). If you use Webpack the best way is to use CopyPlugin.

You must initialize the library before the first use. The best place to do it is in initialization code of your application.

You need to attach the chosen binary module to the `TvmClient` class:

```
import { TvmClient } from '@tvmsdk/core';
import { libWeb } from '@tvmsdk/lib-web';

TvmClient.useBinaryLibrary(libWeb);
```
