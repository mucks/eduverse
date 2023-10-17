const solana = window.solana;


export function tx_from_buffer(buffer) {
    console.log('tx_from_buffer');

    const tx = solanaWeb3.Transaction.from(buffer);

    console.log(tx);
    return tx;
}
