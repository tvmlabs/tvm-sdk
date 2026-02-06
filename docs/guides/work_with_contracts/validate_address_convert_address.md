# Validate address

## When you may need it?

If you want to validate an address.

## Get address type

{% code overflow="wrap" %}
```javascript
let initialAddressType = await client.utils.get_address_type({address});
console.log(`Address type is ${JSON.stringify(initialAddressType)}`);
```
{% endcode %}

## Validate

Use `utils.convert_address` function for that.

```javascript
let convertedAddress = (await client.utils.convert_address({
    address,
    output_format: {
        type: "Hex"
    },
})).address;
console.log(`Address in raw format: ${convertedAddress}`);
```

If address is incorrect the function `utils.convert_address` will fail with an error.
