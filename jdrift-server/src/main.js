const { tauri, app, dialog } = window.__TAURI__;

let message_element = document.querySelector(".message");
let indicator_element = document.querySelector(".indicator");
main().then();

function ms2s(ms) {
    return ms / 1000;
}

function elapsed(s, e, m) {
    let ms = e - s;
    let se = ms2s(ms);
    return `completed after ${se}s: ${m}`;
}

function output(message) {
    console.log(message);
    message_element.innerText = `ok: ${message}`;
}

function output_error(message) {
    console.log(message);
    message_element.innerText = `error: ${message}`;
    indicator_element.style = "display: none";
}

async function main() {
    let port = await tauri.invoke("backend");
    let address = `ws://localhost:${port}`;
    output(`attempting to connect to '${address}'`);

    let start = Date.now();
    let ws = new WebSocket(address);

    ws.onerror = async (error) => {
        let message = `failed to connect to server at '${address}' caused by an error. Click 'open' to open the dev-console for more information`;
        output_error(message);

        let open_dev = await dialog.ask(elapsed(start, Date.now(), message), {
            okLabel: "open",
            type: "error",
            title: "controller socket error"
        });

        if (open_dev) { tauri.invoke('open_devtools'); }
    }

    ws.onopen = () => {
        output(`connected to ${address}. prompt will be replaced once the commander calls the clear method`);
    }
}

