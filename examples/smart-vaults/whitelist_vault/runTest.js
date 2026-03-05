const xrpl = require("xrpl")

const allowed = xrpl.Wallet.fromEntropy([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24], {
  algorithm: xrpl.ECDSA.secp256k1,
})

async function test(testContext) {
  const { deployVault, finish: code, submit, sourceWallet, destWallet, fundWallet } = testContext

  await fundWallet(allowed)
  console.log(`Allowed wallet: ${allowed.address}`)
  const deployResult = await deployVault(sourceWallet, code)

  // deposit into vault
  const depositTx = {
    TransactionType: "VaultDeposit",
    Account: sourceWallet.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "123",
    ComputationAllowance: 1000000,
  }
  const depositResponse = await submit(depositTx, sourceWallet)

  if (depositResponse.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to deposit to vault:",
      depositResponse.result.meta.TransactionResult,
    )
    process.exit(1)
  }

  // withdraw from non-whitelisted account (should fail)
  const failTx = {
    TransactionType: "VaultWithdraw",
    Account: sourceWallet.address,
    Destination: destWallet.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "123",
    ComputationAllowance: 1000000,
  }
  const responseFail = await submit(failTx, sourceWallet)

  if (responseFail.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.log("\nEscrow finished successfully????")
    process.exit(1)
  }

// withdraw from whitelisted account (should succeed)
  const successTx = {
    TransactionType: "VaultWithdraw",
    Account: allowed.address,
    Destination: allowed.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "123",
    ComputationAllowance: 1000000,
  }
  const responseSuccess = await submit(successTx, allowed)

  if (responseSuccess.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to withdraw from vault:",
      responseSuccess.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
