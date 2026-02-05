---
description: Prepared by Pruvendo
---

# Acki Nacki VM Instructions Business-Level Specification

<div align="right"><figure><img src="../../.gitbook/assets/PV logo.webp" alt=""><figcaption></figcaption></figure></div>

## Purpose <a href="#docs-internal-guid-d0a50a9d-7fff-dd98-77a3-5b0f8254351a" id="docs-internal-guid-d0a50a9d-7fff-dd98-77a3-5b0f8254351a"></a>

The purpose of the present document is to create business-level specification (highest-level of specification) for the _Acki Nacki - specific_ VM instructions. This document is intended to:

* Be thoroughly reviewed by the Customer
* Act as a base for the high-level specification

## Introduction <a href="#docs-internal-guid-78137ad4-7fff-57d2-2c7a-211e319816cb" id="docs-internal-guid-78137ad4-7fff-57d2-2c7a-211e319816cb"></a>

Being a TVM-based blockchain, [Acki Nacki](https://www.ackinacki.com/) follows its [specification and instruction set](https://ton.org/tvm.pdf). However, some additional instructions are required to support some specific features introduced in Acki Nacki. Such instructions are described in the present document.

## Acki Nacki - specific instructions <a href="#docs-internal-guid-0e3b4765-7fff-ac3e-9f41-1bee55c7cb64" id="docs-internal-guid-0e3b4765-7fff-ac3e-9f41-1bee55c7cb64"></a>

For all the instructions below the current account is taken from _c4_ register.

### CNVRTSHELLQ <a href="#docs-internal-guid-d761d100-7fff-0c23-f089-995c4fe6dc01" id="docs-internal-guid-d761d100-7fff-0c23-f089-995c4fe6dc01"></a>

Signature : _`C727`_ $$n ( - )$$ , where $$0 \leq n < 2^{256}$$

Brief description: Transforms [_SHELL_](https://docs.ackinacki.com/glossary#shell) into the same amount of [_VMSHELL_](https://docs.ackinacki.com/glossary#vmshell) (using 1:1 ratio)

Input : $$n \bmod 2^{64}$$ - amount of nanotokens to exchange

Behavior:

* If the sender has enough balance of _SHELL_, the required amount of _SHELL_’s are transformed into _VMSHELL_
* Otherwise, the whole balance of _SHELL_ is transformed into _VMSHELL_
* In any case the counter of special operations is increased

### MINTECC <a href="#docs-internal-guid-08520189-7fff-5876-be75-74b8ca24319c" id="docs-internal-guid-08520189-7fff-5876-be75-74b8ca24319c"></a>

Signature : _`C726`_ $$y  \space x ( - )$$, where $$0 \leq x < 256$$, $$0 \leq y < 2^{128}$$

Brief description : mints the required amount of any _ECC_ tokens, can be called by a special contract only

Input : $$x$$ - index of _ECC_ token, $$y$$ - amount of nanotokens to mint

Behavior:

* If the contract is not **special**, _NOT\_SPECIAL\_CONTRACT_ error happens
* If the adding of tokens fails, _OVERFLOW_ error happens
* Otherwise:
  * token balance is increased by the specified amount
  * counter of special actions is increased

### MINTSHELL <a href="#docs-internal-guid-23d7310b-7fff-8379-9be7-f9a162b450b1" id="docs-internal-guid-23d7310b-7fff-8379-9be7-f9a162b450b1"></a>

Signature : _`C728`_ $$n ( - )$$, where $$0 \leq n < 2^{128}$$

Brief description : mints _SHELL_ tokens, up to the specified amount, using the available credit

Input : n - amount of _SHELL_ nanotokens to be minted

Behavior:

* In case of _infinite credit_, the specified amount of _SHELL_ tokens is minted
* Up to the $$n$$ tokens will be minted, but **Minted shell value** afterwards does not exceed available credit
* **Minted shell value** is increased by the amount of the minted value
* In case of success the counter of special actions is increased

All the values must not exceed $$2^{128}$$.

### CALCBKREWARD <a href="#docs-internal-guid-a5808a80-7fff-2166-ca00-6297d9bee59b" id="docs-internal-guid-a5808a80-7fff-2166-ca00-6297d9bee59b"></a>

Signature : _`C729`_ $$r \space s  \space \tau \space  e \space \varSigma \space  n \space a \space (\rho)$$, where:

* $$0 \leq r < 2^{128}$$
* $$0 \leq s < 2^{128}$$
* $$0 \leq \tau < 2^{128}$$
* $$0 \leq e < 2^{128}$$
* $$0 \leq \varSigma < 2^{128}$$
* $$0 \leq n < 2^{128}$$
* $$0 \leq a < 2^{128}$$

Brief description : calculates and returns validator’s reward by the end of each epoch

Input :&#x20;

* $$r$$ - reputation coefficient
* $$s$$ - stake
* $$\tau$$ - total stake at the epoch start
* $$e$$ - epoch duration
* 𝛴- total amount of minted reward tokens
* $$n$$ - number of active block keepers
* $$a$$ - last calculated reward adjustment

Output : $$\rho$$ - assigned reward

Behavior: it follows [Acki Nacki tokenomics](https://tokenomics.ackinacki.com/tokenomics.pdf):&#x20;

* $$\varSigma = 0 \rArr \rho = \frac{aer}{10^9n}$$  &#x20;
* $$0\lt\varSigma \lt TOTALSUPPLY \Rightarrow  \rho = \frac{a e r s}{10^9 \tau}$$
* $$\varSigma \ge TOTALSUPPLY \Rightarrow  \rho =0$$

### CALCMINSTAKE <a href="#docs-internal-guid-f18e0587-7fff-b43e-ba60-86002d69ea5f" id="docs-internal-guid-f18e0587-7fff-b43e-ba60-86002d69ea5f"></a>

Signature : _`C730`_ $$v \space n  \space t \space  s \space  (\mu)$$, where:

* $$0 \leq v < 2^{128}$$
* $$0 \leq n < 2^{128}$$
* $$0 \leq t < 2^{128}$$
* $$0 \leq s < 2^{128}$$

Brief description : calculates a minimal deposit for a validator (in nanotokens)

Input:

* $$v$$ - number of needed active blokkeepers (10000)
* $$n$$ - number of active blokkeepers
* $$t$$ - network duration + ⅓ of the preepoch duration
* $$s$$ - total reward amount subtracted by the total slashed amount

Output : $$\mu$$ - minimally allowed stake

Behavior: it follows [Acki Nacki tokenomics](https://tokenomics.ackinacki.com/tokenomics.pdf)

### CALCBKREWARDADJ <a href="#docs-internal-guid-53350c37-7fff-e909-f366-c5f749fef3c0" id="docs-internal-guid-53350c37-7fff-e909-f366-c5f749fef3c0"></a>

Signature: _`C733`_ $$t \space a \space p \space r \space s \space (𝛼)$$

All the values must not exceed $$2 ^ {128}$$.

Brief description: adjusts reward engine to be aligned with the theoretical curve.

Input:

* $$t$$ - network time
* $$a$$ - the previous adjustment factor
* $$p$$ - reward period
* $$r$$- average reputation coefficient
* $$s$$ - total reward amount

Output : $$a$$ - new adjustment factor

Behavior: it follows [Acki Nacki tokenomics](https://tokenomics.ackinacki.com/tokenomics.pdf)

### CALCREPCOEF <a href="#docs-internal-guid-74742e9d-7fff-4462-583d-5502648353d8" id="docs-internal-guid-74742e9d-7fff-4462-583d-5502648353d8"></a>

Signature: _`C734`_ $$r (c)$$

All the values must not exceed $$2^{128}$$

Brief description: calculates reputation coefficient based on the reputation time.

Input:

* $$r$$ - reputation time

Output : $$c$$ - reputation coefficient

Behavior: it follows [Acki Nacki tokenomics](https://tokenomics.ackinacki.com/tokenomics.pdf)

### Zero-knowledge instructions <a href="#docs-internal-guid-69a99c51-7fff-01b1-0ac6-8c032b755c3f" id="docs-internal-guid-69a99c51-7fff-01b1-0ac6-8c032b755c3f"></a>

One of the common drawbacks of common blockchain systems is a necessity to use seed phrases for authentication. It’s hard to remember them and other (off-chain) approaches such as [OAuth2](https://developers.google.com/identity/protocols/oauth2) that commonly are better in terms or user experience than the former one. The popular solution is to use self-custodial wallets such as [Metamask](https://metamask.io/) or [TON Wallet](https://wallet.ton.org/). Such wallets are often not so transparent as desired and can be vulnerable to attacks (such as the recent [Atomic Wallet case](https://atomicwallet.io/blog/articles/june-3rd-event-statement)).

The provided solution is intended to support popular off-chain authentication systems in blockchain, thus preventing frequent authentication using seed phrases or off-chain wallets, thus combining a high level of security with great user experience.

The solution is based on [Zero-knowledge technology](https://www.rareskills.io/zk-book) and follows the similar solution implemented in [Sui](https://sui.io/) blockchain named _zkLogin_ and described [here](https://docs.sui.io/concepts/cryptography/zklogin) in details, with some important exceptions:

1. Salt is not defined by the user, but simply works as a second password
2. [Poseidon](https://www.poseidon-hash.info/) is used as a hash function
3. The [Ceremony](https://docs.sui.io/concepts/cryptography/zklogin#ceremony) process is completely new

The changes are to be described in a separate document.

### VERGRTH16 <a href="#docs-internal-guid-16cbe58b-7fff-dc84-8535-b9ec05dc8e03" id="docs-internal-guid-16cbe58b-7fff-dc84-8535-b9ec05dc8e03"></a>

Verifies the proofs using Groth16 algorithm

Signature : _`C731`_ $$\pi \space \rho \space i \space (b)$$, where:

* $$\rho \isin TvmSlice$$
* $$d \isin TvmSlice$$
* $$0 \leq i < 2^{32}$$
* $$b \isin B$$

Brief description : checks [zk-Snark](https://z.cash/learn/what-are-zk-snarks/) proof, returning the logical result

Input:

* $$\pi$$ - _slice is public inputs_
* $$\rho$$ - proof (in terms of zk-Snark)
* $$i$$ - algorithm used, where:
  * _0_ - unsecure algorithm
  * _1_ - secure algorithm
  * _anything else_ - test algorithm

Output:

* $$b$$ - boolean value that indicates if the verification was correct or not

Behavior is as follows:

1. $$\pi$$ and $$\rho$$ are decoded into corresponding arrays of bytes
2. proofs then decoded into array of `Proof<Bn254>` structures, using external function
3. public inputs, in their turn, are decoded into _array_ of _Fr_ (where $$Fr = \lbrace 0, ..., 2^{256} -1 \rbrace$$)\
   using:\
   a. external methods for deserialization to the array of `FieldElementWrapper`\
   b. wrapping by the first element
4. Depending on $$i$$, selected the predefined set of verifying keys - **unsecure, secure** or **test**
5. External function with data calculated at steps 2-4 is called to verify the proofs using **Groth16** algorithm&#x20;

### POSEIDON <a href="#docs-internal-guid-3f5e4e8d-7fff-fbe5-f625-82858221accf" id="docs-internal-guid-3f5e4e8d-7fff-fbe5-f625-82858221accf"></a>

Calculates Poseidon hash.

Signature : _`C732`_ $$i \space m \space p \space \mu \space s \space h \space z \space (\pi)$$, where:

* $$0 \leq i < 256$$
* $$0 \leq m < 2^{64}$$
* $$0 \leq p < 2^{256}$$
* $$\mu \isin TvmSlice$$
* $$s \isin TvmSlice$$
* $$h \isin TvmSlice$$
* $$z \isin TvmSlice$$
* $$\pi \isin TvmCell$$

Input:

* $$z$$ - [zk-address](https://docs.sui.io/concepts/cryptography/zklogin#will-my-zklogin-address-ever-change)
* $$h$$ - JWT header (base64)
* $$s$$ - _iss_ (provider’s name) (base64)
* $$\mu$$ - modulus
* $$p$$ - ephemeral public key
* $$m$$ - maximum epoch
* $$i$$ - key index

Output:

* $$\pi$$ - array of public input as a cell

Behavior:

1. Initially:
   1. $$z$$ is transformed into a string (_z-string_)
   2. $$h$$ is transformed into a _base64_ string (_h-string_)
   3. $$s$$ is transformed into a _base64_ string (_s-string_)
   4. $$\mu$$ is transformed into array of bytes (_𝜇-array_)
   5. $$p$$ is transformed into a byte array with proper number of bytes, extra bytes are dropped, in case of lack of bytes exception must be raised
   6. $$m$$ is transformed into _u64_ (_m64_)
   7. $$i$$ is transformed into a _base64_ string (_i-string_)
2. Then:
   1. $$z-string$$ is transformed into big number using an external function
   2. $$h-string$$ is transformed into big number using an external function
   3. $$s-string$$ is transformed into big number using an external function
   4. $$\mu -array$$ is transformed into big number using an external function
   5. $$p-string$$ is split into two big numbers using external function
   6. $$m64$$ is transformed into a string and then, to big number using an external function
   7. $$i-string$$ is transformed into big number using an external function
3. Then, an array of public inputs is created using a call of external functions
4. Finally, the list is serialized into a cell and push into a stack
