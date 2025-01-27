# Endpoint Configuration

## Create TvmClient

`TvmClient` is the main class of TVM SDK Library. To start using library one needs to create and setup a TvmClient instance.

The simplest initialization code can look like this: just specify the endpoint.

Other parameters are used by default. See the reference below for more info.

```javascript
const client = new TvmClient({
network: { 
    endpoints: [
        'your-endpoint-here'
    ] 
    } 
});
```

## Multiple endpoints configuration

If you have multiple endpoints in the same network, you can specify them all.

Library will automatically perform balancing based on endpoint health checks and availability.

```javascript
const client = new TvmClient({
network: { 
    endpoints: [
        'ENDPOINT_URL1', 
        'ENDPOINT_URL2', 
        'ENDPOINT_URL3'
    ] 
    } 
});
```

You can also configure the message broadcast - how many nodes you want your message to be sent (it may improve delivery rate) like this.

```javascript
const client = new TvmClient({
network: { 
    endpoints: [
        'ENDPOINT_URL1', 
        'ENDPOINT_URL2', 
        'ENDPOINT_URL3'
    ] 
    sending_endpoint_count: 3
    } 
});
```
