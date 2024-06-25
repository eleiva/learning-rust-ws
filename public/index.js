const ws = new WebSocket("ws://127.0.0.1:3012");

ws.addEventListener("open", function (event) {
    ws.send("is_connected");
});

function scrollToBottom() {
    const container = document.getElementById('messages');
    container.scrollTop = container.scrollHeight;
}

ws.addEventListener("message", function (event) {
    // Selecciona el elemento 'container'
    const container = document.getElementById('messages');

    let response = JSON.parse(event.data);
    let content;
    let content_available = false;
    let formatted_message = "";

    const message_tmpl = `
        <div class="chat-message">
            <div class="flex items-end">
                <div class="flex flex-col space-y-2 max-w-xs mx-2 order-2 items-start">
                    <div><span class="px-4 py-2 rounded-lg inline-block rounded-bl-none bg-gray-300 text-gray-600">{{message}}</span></div>
                </div>
                <img src="bot.webp"
                    alt="My profile" class="w-6 h-6 rounded-full order-1">
            </div>
        </div>`;

    try {
        content = JSON.parse(response.msg);
        content_available = true;
    } catch (error) {
    }

    if (content_available) {
        console.log(content);

        switch (content.display) {
            case "menu": output = buildMenu(content);
                break;

            case "item_list": output = buildSearch(content);
                break;
            default: output = "No implementado";
        }
    } else {
        output = response.msg;
    }

    // Message from server -> console.log( event.data);
    formatted_message = message_tmpl.replace('{{message}}', output);

    // Inserta el HTML antes del cierre del 'container'
    container.insertAdjacentHTML('beforeend', formatted_message);

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
        //const ws = new WebSocket("ws://127.0.0.1:3012");
        location.reload();
    }, 3000); // Recarga la página después de 3 segundos
});

// Función para manejar errores del WebSocket
ws.addEventListener('error', function (event) {
    console.error('Error en WebSocket: ', event);
    document.getElementById('status').textContent = 'Estado: Error';
});

window.onload = function () {
    document.getElementById("message").focus();
};

function buildMenu(menu) {

    let output = "<ul>";

    menu.menu_items.forEach(function (menu_item) {
        output += "<li>" + menu_item.title + " : <strong>" + menu_item.key + "</strong></li>";
    });

    output += "</ul>"

    return output
}

function buildSearch(search) {

    let output = "<span>" + search.msg + "</span><ul>";

    search.items.forEach(function (item) {
        output += "<a target='_blank' href=" + item.link + "><li style='border: 1px dotted #0000004d;padding:10px;'><strong>" + item.title + "</strong><img src='" + item.images[0] + "' alt='Italian Trulli'><span>$ " + item.price + "</span></li></a>";
    });

    output += "</ul>"

    return output
}