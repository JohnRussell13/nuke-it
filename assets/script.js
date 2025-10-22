let ws = null;
const logEl = document.getElementById("log");

function log(msg) {
  console.log(msg);
  logEl.textContent += msg + "\n";
  logEl.scrollTop = logEl.scrollHeight;
}

document.getElementById("connectBtn").onclick = () => {
  if (ws) {
    log("Already connected");
    return;
  }
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
  if (!ws) {
    log("Not connected");
    return;
  }
  const msg = { action: "spin", wallet_id: "123", player_id: 1, amount: 1.3 };
  ws.send(JSON.stringify(msg));
  log("Sent: " + JSON.stringify(msg));
};

document.getElementById("fetchBtn").onclick = () => {
  if (!ws) {
    log("Not connected");
    return;
  }
  const msg = { action: "fetch" };
  ws.send(JSON.stringify(msg));
  log("Sent: " + JSON.stringify(msg));
};

document.getElementById("disconnectBtn").onclick = () => {
  if (!ws) {
    log("Not connected");
    return;
  }
  ws.close();
  ws = null;
};
