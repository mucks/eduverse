const provider = window.solana;

const tx = new solanaWeb3.Transaction();

provider.signTransaction(tx);