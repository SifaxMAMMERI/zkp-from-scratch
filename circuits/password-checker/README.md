# Password Checker Circuit

## Overview

This circuit verifies whether a secret password matches a known hash without revealing the password.

It demonstrates a basic authentication mechanism using Zero-Knowledge Proofs.

---

## Inputs

- **Private input**
  - `password`: the secret value

- **Public input**
  - `storedHash`: expected hash of the password

---

## Logic

1. The password is hashed using the Poseidon hash function  
2. The computed hash is compared with the stored hash  
3. The circuit outputs:  
   - `1` if they match  
   - `0` otherwise  

---

## Concepts

- Poseidon hash (ZKP-friendly hash)  
- Equality constraints  
- Public vs private inputs  
- Basic ZKP authentication model  
