async function test(testContext) {
  const { deployVault, finish, submit, sourceWallet, destWallet } = testContext

  const deployResult = await deployVault(sourceWallet, destWallet, finish)

  // deposit into escrow
  const depositTx = {
    TransactionType: "VaultDeposit",
    Account: sourceWallet.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "123",
    ComputationAllowance: 1000000,
  }
  const depositResponse = await submit(depositTx, sourceWallet, true)

  if (depositResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to deposit to vault:",
      depositResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }

    // deposit into escrow
  const withdrawTx = {
    TransactionType: "VaultWithdraw",
    Account: sourceWallet.address,
    Destination: destWallet.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "123",
    ComputationAllowance: 1000000,
  }
  const withdrawResponse = await submit(withdrawTx, sourceWallet, true)

  if (withdrawResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to withdraw from vault:",
      withdrawResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
