const CONTRACT_ID = "CD7KVIAGEMJCOZAPW23IWQLGTKGZXY2QS3AN7WPAOO4PB7DQERTDD6BY";
const RPC_URL = "https://soroban-testnet.stellar.org";

let walletAddress = null;

// Wait for Freighter to inject then try to get it
function getFreighter() {
  return window.freighterApi || window.freighter || window.stellar;
}

async function connectWallet() {
  // Wait a bit for extension to inject
  await new Promise(r => setTimeout(r, 500));
  
  const api = getFreighter();
  console.log("Freighter API found:", api);

  if (!api) {
    alert("Freighter not detected! Try: \n1. Refresh the page \n2. Make sure Freighter is enabled \n3. Try opening Freighter extension first then connect");
    return;
  }

  try {
    await api.requestAccess();
    walletAddress = await api.getPublicKey();
    document.getElementById("walletAddress").innerText = "Connected: " + walletAddress;
    document.getElementById("connectBtn").innerText = "✅ Wallet Connected";
    loadTransactions();
  } catch (e) {
    console.error(e);
    alert("Failed to connect: " + e.message);
  }
}

async function addTransaction() {
  if (!walletAddress) {
    alert("Please connect your wallet first!");
    return;
  }

  const farmer = document.getElementById("farmer").value;
  const crop = document.getElementById("crop").value;
  const quantity = parseInt(document.getElementById("quantity").value);
  const price = parseInt(document.getElementById("price").value);
  const date = document.getElementById("date").value;

  if (!farmer || !crop || !quantity || !price || !date) {
    alert("Please fill in all fields!");
    return;
  }

  document.getElementById("status").innerText = "Submitting transaction...";

  try {
    const { Contract, SorobanRpc, TransactionBuilder, Networks, BASE_FEE, nativeToScVal } = StellarSdk;
    const server = new SorobanRpc.Server(RPC_URL);
    const contract = new Contract(CONTRACT_ID);
    const account = await server.getAccount(walletAddress);

    const tx = new TransactionBuilder(account, {
      fee: BASE_FEE,
      networkPassphrase: Networks.TESTNET,
    })
      .addOperation(
        contract.call(
          "add_transaction",
          nativeToScVal(farmer, { type: "string" }),
          nativeToScVal(crop, { type: "string" }),
          nativeToScVal(quantity, { type: "u32" }),
          nativeToScVal(price, { type: "u32" }),
          nativeToScVal(date, { type: "string" })
        )
      )
      .setTimeout(30)
      .build();

    const preparedTx = await server.prepareTransaction(tx);
    const api = getFreighter();
    const signedXDR = await api.signTransaction(
      preparedTx.toXDR(),
      { networkPassphrase: Networks.TESTNET }
    );

    const { TransactionBuilder: TB, Networks: N } = StellarSdk;
    const signedTx = TB.fromXDR(signedXDR, N.TESTNET);
    await server.sendTransaction(signedTx);

    document.getElementById("status").innerText = "✅ Transaction recorded on blockchain!";
    loadTransactions();
  } catch (e) {
    console.error(e);
    document.getElementById("status").innerText = "❌ Error: " + e.message;
  }
}

async function loadTransactions() {
  try {
    const { Contract, SorobanRpc, TransactionBuilder, Networks, BASE_FEE, scValToNative } = StellarSdk;
    const server = new SorobanRpc.Server(RPC_URL);
    const contract = new Contract(CONTRACT_ID);

    const sourceAccount = await server.getAccount(
      walletAddress || "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN"
    );

    const tx = new TransactionBuilder(sourceAccount, {
      fee: BASE_FEE,
      networkPassphrase: Networks.TESTNET,
    })
      .addOperation(contract.call("get_transactions"))
      .setTimeout(30)
      .build();

    const result = await server.simulateTransaction(tx);
    const transactions = scValToNative(result.result.retval);

    const tbody = document.getElementById("txTable");
    if (!transactions || transactions.length === 0) {
      tbody.innerHTML = '<tr><td colspan="5" style="text-align:center">No records yet</td></tr>';
      return;
    }

    tbody.innerHTML = transactions.map(t => `
      <tr>
        <td>${t.farmer}</td>
        <td>${t.crop}</td>
        <td>${t.quantity}</td>
        <td>${t.price}</td>
        <td>${t.date}</td>
      </tr>
    `).join("");
  } catch (e) {
    console.error("Failed to load transactions:", e);
  }
}