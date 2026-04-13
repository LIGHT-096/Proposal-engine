# 🚀 Proposal Engine on Stellar (Soroban)
<img width="1920" height="1020" alt="Screenshot 2026-04-13 135941" src="https://github.com/user-attachments/assets/40947a7d-fee6-4a24-bbb8-6e2410583de7" />


## 📌 Project Description

The **Proposal Engine** is a simple smart contract built using **Soroban (Stellar's smart contract platform)**. It allows users to create proposals and vote on them in a decentralized manner.

This project demonstrates the basics of building, storing, and interacting with on-chain data using Soroban.

---

## ⚙️ What It Does

* Users can create proposals with a unique ID and title
* Anyone can vote on existing proposals
* The contract stores vote counts on-chain
* Users can fetch proposal details anytime

---

## ✨ Features

* 🧾 Create proposals
* 🗳️ Vote on proposals
* 📊 Track vote counts
* 🔍 Retrieve proposal data
* ⚡ Lightweight and fast (Soroban-based)

---

## 🛠️ Tech Stack

* **Rust** (Smart Contract Language)
* **Soroban SDK**
* **Stellar Blockchain**

---

## 🔗 Deployed Smart Contract Link

> ⚠️ Replace this with your deployed contract link

```
https://stellar.expert/explorer/testnet/contract/CBK3N356XXWHCC4BA4CIDPK5JKC7CEILYFSL6UXK5NZOYNTSMYZ75TW5
```

---

## 🚀 How to Run Locally

1. Install Soroban CLI:

```
cargo install --locked soroban-cli
```

2. Build the contract:

```
cargo build --target wasm32-unknown-unknown --release
```

3. Deploy to testnet:

```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/your_contract.wasm \
  --source YOUR_ACCOUNT
```

---

## 📈 Future Improvements

* 🧑‍⚖️ Add proposal creator identity
* ⛔ Prevent double voting
* ⏳ Add voting deadlines
* 🏆 Winner selection logic
* 🌐 Frontend integration

---

## 👨‍💻 Author

Your Name

---

## 📜 License

MIT License
