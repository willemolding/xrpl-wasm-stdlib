const xrpl = require("xrpl")

async function test(testContext) {
  const { deployVault, finish: code, submit, sourceWallet, destWallet, fundWallet } = testContext

  const deployResult = await deployVault(sourceWallet, code, "00")

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

  // withdraw from non-depositor (should fail)
  const failTx1 = {
    TransactionType: "VaultWithdraw",
    Account: destWallet.address,
    Destination: destWallet.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "123",
    ComputationAllowance: 1000000,
  }
  const responseFail1 = await submit(failTx1, destWallet)

  if (responseFail1.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error("\nWithdraw succeeded that should have failed")
    process.exit(1)
  }

    // withdraw the wrong amount (should fail)
  const failTx2 = {
    TransactionType: "VaultWithdraw",
    Account: sourceWallet.address,
    Destination: sourceWallet.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "999",
    ComputationAllowance: 1000000,
  }
  const responseFail2 = await submit(failTx2, sourceWallet)

  if (responseFail2.result.meta.TransactionResult !== "tecWASM_REJECTED") {
    console.error("\nWithdraw succeeded that should have failed")
    process.exit(1)
  }

  // withdraw correct amount from depositor (should succeed)
  const successTx = {
    TransactionType: "VaultWithdraw",
    Account: sourceWallet.address,
    Destination: sourceWallet.address,
    VaultID: deployResult.vaultKeylet,
    Amount : "123",
    ComputationAllowance: 1000000,
  }
  const responseSuccess = await submit(successTx, sourceWallet)

  if (responseSuccess.result.meta.TransactionResult !== "tesSUCCESS") {
    console.error(
      "\nFailed to withdraw from vault:",
      responseSuccess.result.meta.TransactionResult,
    )
    process.exit(1)
  }
}

module.exports = { test }
