# did-miner

Change the string (`let contains = "1ota";`) to whatever you want to mine and run it with `cargo run --release`.

Mining for strings longer than 4 characters can quickly take a long time.

let position = Position::Start; looks like:

`Found a match: private key: "DDbG9RHYG31kKQZK2Za5jirbsXgjLEiw55BmHwmR8Hws", public key: "D3U1sDmvRhACQYQjbkKQzTn9mMNWj4nokwXEq2KBoBsV", did_tag: "`1ota`EsLSxJHTPxLx6gXFENsYV1EyycWDnLcDj7RzrF6"`

let position = Position::Any; looks like:

`Found a match: private key: "6RaBDzHc4wsM5gG7EvchcqCNcH2koK8XN6Xva5kzckrT", public key: "7jcBfuwLPJbs3q1T9vc2kkJ1VgpWA4dUkfq7ngc1akLc", did_tag: "CV3d6CxshxsZyq2bnwLwCQJy`1ota`Rjot5sK42fMWFntG"`


To create a DID document with the keypair the following code can be used in node.js with `@iota/identity-wasm/node/`

```JS
const identity = require('@iota/identity-wasm/node')

const run_testnet = async () => {

    const key = identity.KeyPair.fromBase58(1, "D3U1sDmvRhACQYQjbkKQzTn9mMNWj4nokwXEq2KBoBsV", "DDbG9RHYG31kKQZK2Za5jirbsXgjLEiw55BmHwmR8Hws")


    const did = new identity.DID(key, "test") // <---- "test" or "main"

    console.log(did)

    const method = identity.VerificationMethod.fromDID(did, key, "authentication")

    const didDoc = identity.Document.fromAuthentication(method)

    didDoc.sign(key)

    message = await identity.publish(didDoc.toJSON(), { node: "https://api.lb-0.h.chrysalis-devnet.iota.cafe/", network: "test" })

    console.log(message)

    document = await identity.resolve(didDoc.id.toString(), { node: "https://api.lb-0.h.chrysalis-devnet.iota.cafe/", network: "test" })

    console.log(document)

}

run_testnet()
```
