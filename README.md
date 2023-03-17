<br />
<p align="center">
  <img src="https://cavies.xyz/assets/images/logo.png" alt="CaviesLabs" />
</p>

<h3 align="center">
  <strong>Built for flexible, customizable Self-managed DCA strategies #defi #infrastructure 
</strong>
</h3>

<p align="center">
     <a href="https://pocket.hamsterbox.xyz">
        Launch DApp
    </a> |
    <a href="https://cavies.xyz/">
        About Cavies
    </a>
</p>

<p align="center">

</p>

<p align="center">
  <a href="https://github.com/CaviesLabs/hamsterpocket-program/">
    <img alt="GitHub Repository Stars Count" src="https://img.shields.io/github/stars/CaviesLabs/hamsterpocket-program?style=social" />
  </a>
    <a href="https://twitter.com/CaviesLabs">
        <img alt="Follow Us on Twitter" src="https://img.shields.io/twitter/follow/CaviesLabs?style=social" />
    </a>
    <a href="https://linkedin.com/company/cavieslabs">
        <img alt="Follow Us on LinkedIn" src="https://img.shields.io/badge/LinkedIn-Follow-black?style=social&logo=linkedin" />
    </a>
</p>
<p align="center">
    <a href="#">
        <img alt="Build Status" src="https://build.cavies.xyz/buildStatus/icon?job=hamsterpocket%2Fhamsterpocket-backend%2Fdevelop" />
    </a>
    <a href="https://github.com/CaviesLabs/hamsterpocket-program">
        <img alt="License" src="https://img.shields.io/github/license/CaviesLabs/hamsterpocket-program" />
    </a>
    <a href="https://github.com/CaviesLabs/hamsterpocket-program/pulls">
        <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg" />
    </a>
    <a href="https://coveralls.io/github/CaviesLabs/hamsterpocket-program/?branch=next">
        <img alt="Coverage Status" src="https://coveralls.io/repos/github/CaviesLabs/hamsterpocket-program/badge.svg?branch=next" />
    </a>
</p>

![Hero image](https://files.slack.com/files-pri/T03N86YEZ6Z-F04TQLW6JCU/heroimage.png?pub_secret=014779ae87)

Hamsterpocket (Pocket) is an Open Source self-managed dollar-cost-averaging protocol that lets users create and run their own saving pools (“pockets”) that will automatically execute the chosen strategies over time.

## **What we deliver out-of-the-box** 📦

<p align="center">
  <img alt="Architecture" src="https://files.slack.com/files-pri/T03N86YEZ6Z-F04T783JZAB/out-of-the-box.png?pub_secret=3ca2221066">
</p>

- **Convenient** - Users only need to set up the desired pools and strategies once, then top up (reload) said pools with the required funds for execution anytime.
- **Trustless** - Users are able to manage their own pools. Every pocket can only be paused, resumed, terminated and withdrawn at will by the pockets’ owners.
- **Flexible use cases**:

  • Run a TWAP strategy on-chain

  • Create a simple saving pool for one or multiple assets

  • Set-and-forget vaults for medium or long-term spot purchases

## **Prototype Design & Test cases** 🚴

- [**Figma**](https://www.figma.com/file/Tx32sB0eC2iwkBD7rZRRut/Hamsterpocket-(DCA)?node-id=1902%3A43690&t=JpssstVDMVaaWHSf-0)
- [**Test cases**](https://docs.google.com/spreadsheets/d/1xdPHErMtTJtk0zH2-huzkDcuJx9lwZgb/edit#gid=1292533007)

## **Our Tech Stack** 🛠

- [ReactJs](https://reactjs.org/)
- [Next.js](https://nextjs.org/)
- [Nest.js](https://nestjs.com/)
- [TypeScript](https://www.typescriptlang.org/)
- [Yarn](https://yarnpkg.com/)
- [Docker](https://www.docker.com/)
- [Anchor](https://anchor-lang.com/)
- [Rust](https://rustup.rs/)

## **Related Workspaces** 🔗

- [hamsterpocket-backend](https://github.com/CaviesLabs/hamsterpocket-backend) - Backend repository
- [hamsterpocket-frontend](https://github.com/CaviesLabs/hamsterpocket-frontend) - Frontend repository/DApp
- [hamsterpocket-program](https://github.com/CaviesLabs/hamsterpocket-program) - Rust smart contract repository

## **Getting started** 🚀

### **Step 0. Deployed Contracts addresses** 📢
| Mainnet | Devnet       |
| ------------- |--------------|
| BW5RwMCPY85ch6efYE3Ev43ZQpJytvvjSNbJ2beC9MzV | BW5RwMCPY85ch6efYE3Ev43ZQpJytvvjSNbJ2beC9MzV |

Raydium markets API: https://api.raydium.io/v2/sdk/liquidity/mainnet.json

### **Step 1: Clone this repo** 🧰
Create a new Hamsterpocket project with Open Source integration in just a few steps.

```bash
git clone https://github.com/CaviesLabs/hamsterpocket-program.git <project_name>
```

### **Step 2: Install dependencies** ⏳

Go to the newly created directory and install the required dependencies:


```bash
cd <project_name>

yarn
```

> Hamsterpocket supports only **Yarn** package manager. If you don't have it installed, please follow the [official Yarn installation guide](https://yarnpkg.com/getting-started/install).

### **Step 3: Configure the project** ⚙️

Please refer to the `.env.example` to adjust your desired configuration. Please contact <a href="mailto:khang@cavies.xyz">khang@cavies.xyz</a> or  <a href="mailto:dev@cavies.xyz">dev@cavies.xyz</a> for any inquiries.

### **Step 4: Test the program** 🧪

Run the solana test validator
```bash
solana-test-validator --no-bpf-jit --reset
```

Run the test
```bash
$ anchor test --skip-local-validator

#  pocket_registry
#    ✔ [initialize_swap_program] should: deployer initializes pocket registry successfully
#    ✔ [initialize_swap_program] should: deployer fails to re-initialize the pocket registry
#    ✔ [update_operator] should: deployer can update operators list (453ms)
#
#  pocket
#    ✔ [create_pocket] should: anyone can create their pocket (477ms)
#    ✔ [create_token_vault] should: pocket owner can create token vault successfully (473ms)
#    ✔ [pause_pocket] should: owner should pause pocket successfully (469ms)
#    ✔ [pause_pocket] should: owner should not pause pocket that was already paused
#    ✔ [restart_pocket] should: owner can restart the paused pocket successfully (436ms)
#    ✔ [close_pocket] should: owner can close pocket successfully (470ms)

#  assets
#    ✔ [deposit] should: owner can deposit assets to pocket successfully (485ms)
#    ✔ [withdraw] should: owner can withdraw assets from pocket successfully (935ms)
```

### **Step 4: Deploy the programs** 🔥

(1) Get new program address 

```bash
anchor build
solana address -k target/deploy/pocket-keypair.json

# above program will output the address of program
# replace all `BW5RwMCPY85ch6efYE3Ev43ZQpJytvvjSNbJ2beC9MzV` with your new address
```

(2) Deploy the program on mainnet

```bash
anchor deploy --program-name pocket --provider.cluster mainnet-beta \
--provider.wallet ~/.config/solana/id.json
```

## **Contribution** 🤝

Hamsterpocket is an Open Source project and we encourage everyone to help us making it better. If you are interested in contributing to the project, please feel free to do so.

If you have any questions about contributing, please refer to our twitter <a href="https://twitter.com/CaviesLabs">
<img alt="Follow Us on Twitter" src="https://img.shields.io/twitter/follow/CaviesLabs?style=social" />
</a> - we are happy to help you!

Discovered a 🐜 or have feature suggestion? Feel free to [create an issue](https://github.com/CaviesLabs/hamsterpocket-program/issues/new/choose) on Github.

## **Support us** ❤️

**Hamsterpocket is and always will be Open Source, released under MIT Licence.**

How you can help us:

- **Contribute** - this is how the Core Team is supporting the project.
- **Spread the word** - tell your friends, colleagues, and followers about Hamsterpocket.
- **Create content** - write a blog post, record a video, or create a tutorial. We will be happy to share it on our social media channels.

### **Follow us on Social Media**

[![Twitter Follow](https://img.shields.io/twitter/follow/CaviesLabs?style=social)](https://twitter.com/CaviesLabs)
[![LinkedIn Follow](https://img.shields.io/badge/LinkedIn-Follow-black?style=social&logo=linkedin)](https://www.linkedin.com/company/cavieslabs/)

## **Careers** 👩‍💻👨‍💻

We are growing and we are looking for talented people to join our team. If you are interested in working with us, please check our [Careers page](https://www.notion.so/cavies/Job-Board-320ac7987dc64a53b0d3d3e7c52c5ce7).

## **Contacts** 📱📱

Feel free to submit your inquiries to <a href="mailto:dev@cavies.xyz">dev@cavies.xyz</a> or <a href="mailto:hello@cavies.xyz">hello@cavies.xyz</a>
