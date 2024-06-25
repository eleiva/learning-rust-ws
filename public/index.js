const ws = new WebSocket("ws://127.0.0.1:3012");

ws.addEventListener("open", function (event) {
    console.log("Sending message to server: Meow!")
});

function scrollToBottom() {
    const container = document.getElementById('messages');
    container.scrollTop = container.scrollHeight;
}

ws.addEventListener("message", function (event) {
    // Selecciona el elemento 'container'
    const container = document.getElementById('messages');

    let response = JSON.parse(event.data);

    console.log(response);
    // Crea un nuevo elemento
    const newMessage = `
        <div class="chat-message">
            <div class="flex items-end">
                <div class="flex flex-col space-y-2 max-w-xs mx-2 order-2 items-start">
                    <div><span
                            class="px-4 py-2 rounded-lg inline-block rounded-bl-none bg-gray-300 text-gray-600">{{message}}</span></div>
                </div>
                <img src="bot.webp"
                    alt="My profile" class="w-6 h-6 rounded-full order-1">
            </div>
        </div>`;

    // Message from server -> console.log( event.data);
    const formattedMessage = newMessage.replace('{{message}}', response.msg);

    // Inserta el HTML antes del cierre del 'container'
    container.insertAdjacentHTML('beforeend', formattedMessage);

    // Scroll to bottom
    scrollToBottom();

});

const sendButton = document.getElementById("send");

sendButton.addEventListener("click", (event) => {
    const message = document.getElementById("message");

    if (message.value.length > 0) {
        ws.send(message.value);

        // Selecciona el elemento 'container'
        const container = document.getElementById('messages');

        // Crea un nuevo elemento
        const newMessage = `
         <div class="chat-message">
            <div class="flex items-end justify-end">
                <div class="flex flex-col space-y-2 max-w-xs mx-2 order-1 items-end">
                <div>
                        <span class="px-4 py-2 rounded-lg inline-block rounded-br-none bg-blue-600 text-white ">{{message}}</span>
                    </div>
                </div>
                <img src="someone.webp" alt="Someone" class="w-6 h-6 rounded-full order-2">
            </div>
        </div>`;

        // Message from server
        const formattedMessage = newMessage.replace('{{message}}', message.value);

        // Inserta el HTML antes del cierre del 'container'
        container.insertAdjacentHTML('beforeend', formattedMessage);

        message.value = '';
    }
});

function handleKeyPress(event) {
    if (event.key === 'Enter') {
        sendButton.click();
    }
}

document.addEventListener('keydown', handleKeyPress);


// Función para manejar la clausura de la conexión WebSocket
ws.addEventListener('close', function (event) {
    console.log('WebSocket se ha cerrado. Recargando la página...');
    // document.getElementById('status').textContent = 'Estado: Desconectado';    
    setTimeout(function () {
        const ws = new WebSocket("ws://127.0.0.1:3012");

//        location.reload();
    }, 3000); // Recarga la página después de 3 segundos
});

// Función para manejar errores del WebSocket
ws.addEventListener('error', function (event) {
    console.error('Error en WebSocket: ', event);
    document.getElementById('status').textContent = 'Estado: Error';
});

window.onload = function() {
    document.getElementById("message").focus();
  };