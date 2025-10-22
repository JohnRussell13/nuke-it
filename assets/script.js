let ws = null;
const logEl = document.getElementById("log");

function log(msg) {
  console.log(msg);
  logEl.textContent += msg + "\n";
  logEl.scrollTop = logEl.scrollHeight;
}

document.getElementById("connectBtn").onclick = () => {
  if (ws) return log("Already connected");
  ws = new WebSocket("ws://127.0.0.1:3000/ws");

  ws.onopen = () => log("WebSocket connected");
  ws.onmessage = (event) => log("Received: " + event.data);
  ws.onclose = () => {
    log("WebSocket disconnected");
    ws = null;
  };
  ws.onerror = (err) => log("WebSocket error: " + err);
};

document.getElementById("spinBtn").onclick = () => {
  if (!ws) return log("Not connected");

  const wallet_id = document.getElementById("walletId").value.trim();
  const game_id = parseInt(document.getElementById("gameId").value);
  const player_id = parseInt(document.getElementById("playerId").value);
  const amount = parseInt(document.getElementById("amount").value);

  if (!wallet_id) {
    log("Invalid input: wallet_id must be a non-empty string");
    return;
  }
  if (
    Number.isNaN(game_id) ||
    Number.isNaN(player_id) ||
    Number.isNaN(amount)
  ) {
    log("Invalid input: game_id, player_id, and amount must be integers");
    return;
  }

  const msg = { action: "spin", wallet_id, game_id, player_id, amount };
  ws.send(JSON.stringify(msg));
  log("Sent: " + JSON.stringify(msg));
};

document.getElementById("fetchBtn").onclick = () => {
  if (!ws) return log("Not connected");

  const wallet_id = document.getElementById("walletId").value.trim();
  const game_id = parseInt(document.getElementById("gameId").value);

  if (!wallet_id) {
    log("Invalid input: wallet_id must be a non-empty string");
    return;
  }
  if (Number.isNaN(game_id)) {
    log("Invalid input: game_id must be an integer");
    return;
  }

  const msg = { action: "fetch", wallet_id, game_id };
  ws.send(JSON.stringify(msg));
  log("Sent: " + JSON.stringify(msg));
};

document.getElementById("disconnectBtn").onclick = () => {
  if (!ws) return log("Not connected");
  ws.close();
  ws = null;
};
