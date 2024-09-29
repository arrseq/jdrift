const { tauri, app, dialog } = window.__TAURI__;

let html = `
    <img width="150px" src="assets/logo.svg" alt="Logo">
    <span class="message">If this message is being displayed, then 'main.js' did not load yet.</span>
    
    <div class="indicator">
        <span></span>
        <span></span>
        <span></span>
        <span></span>
        <span></span>
    </div>
`;

let do_not_show_again = false;
window.onload = () => main().then();

function ms2s(ms) {
    return ms / 1000;
}

function elapsed(s, e, m) {
    let ms = e - s;
    let se = ms2s(ms);
    return `completed after ${se}s: ${m}`;
}

async function main() {
    document.body.innerHTML = html;

    let message_element = document.querySelector(".message");
    let indicator_element = document.querySelector(".indicator");

    function output(message) {
        console.log(message);
        message_element.innerText = `ok: ${message}`;
        message_element.style.background = "transparent";
    }

    function ok(message) {
        console.log(message);
        message_element.innerText = `ok: ${message}`;
        message_element.style.background = "#4AFF5C";
    }

    function output_error(message) {
        console.log(message);
        message_element.innerText = `error: ${message}`;
        message_element.style.background = "#f30038";
        indicator_element.style = "display: none";
    }

    let port = await tauri.invoke("backend");
    let address = `ws://localhost:${port}`;
    output(`attempting to connect to '${address}'`);

    let start = Date.now();
    let ws = new WebSocket(address);

    ws.onerror = async (error) => {
        let message = `failed to connect to server at '${address}' caused by an error. Click 'open' to open the dev-console for more information`;
        output_error(message);

        if (do_not_show_again) return;
        do_not_show_again = true;

        let open_dev = await dialog.ask(elapsed(start, Date.now(), message), {
            okLabel: "open",
            cancelLabel: "no",
            type: "error",
            title: "controller socket error"
        });
        if (open_dev) { tauri.invoke('open_devtools'); }
    }

    ws.onopen = () => {
        ok(`connected to ${address}. prompt will be replaced once the commander calls the clear method`);
    }

    ws.onclose = () => {
        output_error(`Connection to command center was closed. Attempting to reconnect in 1s`);
        setTimeout(() => main().then(), 1000);
    }
}

