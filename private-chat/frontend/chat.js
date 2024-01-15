const ws = new WebSocket("ws://localhost:8080");

ws.onopen = function() {
    console.log("Connected to WebSocket server.");
};

ws.onmessage = function(event) {
    console.log("Message received: ", event.data);
    const message = JSON.parse(event.data);
    const messageElement = document.createElement("p");
    messageElement.textContent = `${message.username}: ${message.content}`;
    document.getElementById("message-area").appendChild(messageElement);
};

ws.onclose = function(event) {
    console.log("WebSocket is closed now.", event);
};

ws.onerror = function(event) {
    console.error("WebSocket error observed:", event);
};

document.getElementById("send-button").addEventListener('click', () => {
    const messageInput = document.getElementById("message-input");
    const message = {
        username: "User", // This should be replaced with the actual username
        content: messageInput.value
    };
    ws.send(JSON.stringify(message));
    messageInput.value = '';
});
