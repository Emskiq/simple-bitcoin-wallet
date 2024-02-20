## Simple Bitcoin Wallet Address to QR Generator

This project is the result of completing the [Simple Bitcoin Wallet in Rust tutorial](https://www.youtube.com/watch?v=md-ecvXBGzI).

As stated in my previous [Rustling Exercises repository](https://github.com/Emskiq/rustling-exercises), this is the second step in my learning Rust path: Doing simple to intermediate projects in Rust to enhance my understanding of the language.

I'm actively consolidating my understanding by documenting the functionality and changes made during the development process (with a bit of copy and paste 😅).

---

### Preview
![](simple-bitcoin-wallet.gif)

---

### Usage

#### Prerequisite:
- Node.js installed
- npm installed
- Rust installed
- Protoc installed (`apt install protobuf-compiler`)

#### Run the backend

In the main directory of the project:
```bash
cargo run
```
This will build the rust program as long as all the dependencies.

#### Run the front end

In the `frontend` directory of the project run:
```bash
npm install # or pnpm install or yarn install
```
This will install the frontend website, after that you need to run it:
```bash
npm start
```

### References and Changes Made

Although Paul from [PlebLabs](https://github.com/PlebLab/PlebLab_Workshops/issues/20) created a fantastic tutorial with a ton of valuable insights and explanations, I encountered issues with the project from the [original repository](https://github.com/futurepaul/paypaul).

Also, I've attempted to _continue_ the tutorial in my README format, sharing my struggles and the materials I've found along the way. With that being said, I aim to build on top of the state in which the tutorial ends.
