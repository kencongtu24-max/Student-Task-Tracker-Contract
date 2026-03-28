# 🎓 Student Task Tracker Contract

A decentralized task management smart contract built on Stellar Blockchain using Soroban Smart Contracts.

---

## 📖 Description

Student Task Tracker Contract is a simple decentralized application (dApp) built on the Stellar blockchain to help users store and manage a study task or personal task directly on-chain.

### The Problem

Students often need to keep track of assignments, reports, projects, and daily study goals. Traditional note-taking tools are useful, but they can be edited, deleted, or lost, and they do not provide a transparent way to track task progress.

### The Solution

By storing a task on-chain through a Soroban smart contract, this system ensures that:

- The current task is stored transparently on the blockchain
- Users can check their current task anytime
- The completion status is managed clearly through the contract
- The task can be reset to start a new one
- The project demonstrates how blockchain can be used for simple productivity tools

### Why Stellar?

Stellar was chosen because it offers fast transaction finality, very low transaction fees, and support for Soroban smart contracts. These features make it a strong platform for building lightweight and practical blockchain applications.

---

## ✨ Features

### 📝 Set Task
- Users can create or update a new task
- When a new task is created, the completion status is automatically reset to `false`

### 📋 Get Task
- Users can view the current task stored in the contract
- If no task has been created yet, the contract returns `"No task yet"`

### ✅ Complete Task
- Users can mark the current task as completed
- The contract updates the status to `true`

### 🔍 Get Status
- Users can check whether the current task is completed
- Returns `true` or `false`

### 🔄 Reset Task
- Resets the task to its default state
- After reset, the task becomes `"No task yet"` and the completion status becomes `false`

### 🧪 Contract Testing
- The contract was successfully tested using `cargo test`
- Test result: `1 passed, 0 failed`

---

## 📄 Contract

**Network:** Stellar Testnet

**Contract Address:**

`CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL`

🔍 **View on Stellar Expert:**  
[https://stellar.expert/explorer/testnet/contract/CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL](https://stellar.expert/explorer/testnet/contract/CAX72JBBXQSZUOHBOJ44M2AYJ3YDITWHZTQSPY5O4QFLOPY7XSZAYLKL)

---

## 📸 Contract Screenshot

<img width="1024" height="666" alt="Ảnh màn hình 2026-03-27 lúc 22 35 26" src="https://github.com/user-attachments/assets/8144be18-e652-45a0-b194-4877e29e8578" />

---

## 🚀 Future Scopes

This project was built as a simple educational smart contract, but it can be expanded in many practical directions in the future.

### Short-term
- Support multiple tasks instead of only one
- Add task creation timestamps
- Add task priority levels
- Keep a history of completed tasks

### Mid-term
- Connect with Stellar wallets so each user can manage their own tasks
- Build a frontend dApp for a better user experience
- Organize tasks by subject, project, or category

### Long-term
- Develop it into a decentralized study management platform
- Add a reward or point system for completed tasks
- Build a transparent on-chain productivity tool for students

The long-term goal is to create a simple but meaningful on-chain learning assistant where students can track their study progress in a transparent and decentralized way.

---

## 👤 Profile

| Field | Information |
|-------|-------------|
| Name | Vo Ba Quoc Dat |
| ID | 113400 |
| School | Dong A University |
| Major | Information Technology — Blockchain |
| Year | 1rd Year |

Built with ❤️ at Stellar Blockchain Workshop · Powered by Soroban Smart Contracts
