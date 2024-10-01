import { Decoder } from "xbinser/src/lib";
import {create, Kind} from "./main/create";
import {delete_element} from "./main/delete";
import {set_text} from "./main/set_text";
import {set_property} from "./main/set_property";

function tauri_ask(message: string, options: any): boolean {
    if (!(window as any).__TAURI__) return;
    const { dialog } = (window as any).__TAURI__;
    dialog.ask(message, options);
}

let params = new URLSearchParams(window.location.search);

function tauri_invoke(channel: string): number {
    let backend = params.get("port");
    if (channel == "backend" && backend) return Number(backend);

    if (!(window as any).__TAURI__) return;
    const { tauri } = (window as any).__TAURI__;
    return tauri.invoke(channel);
}

let html = `
    <div class="box">
        <div class="indicator"></div>
        <span class="message">If this message is being displayed, then 'main.js' did not load yet.</span>
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

let decoder = new Decoder({
    class: "u32",
    kind: [
        "enum[",
            "create[",
                "parent: u32, ",
                "kind: enum[division[], span[], paragraph[], button[], header[]]",
            "], ",
            "delete[], ",
            "set_text[text: string], ",
            "set_property[kind: enum[style[], attribute[]], property: string, value: string]",
        "]"
    ].join("")
});

export interface PropertyKind {
    style?: {};
    attribute?: {};
}

interface MessageKind  {
    create?: { parent: number, kind: Kind };
    delete?: {};
    set_text?: { text: string };
    set_property?: { kind: PropertyKind, property: string, value: string };
}

interface Message {
    class: number;
    kind: MessageKind;
}

async function main() {
    document.body.innerHTML = html;
    (document.body as any).style = "";

    let message_element = document.querySelector(".message");

    function output(message) {
        console.log(message);
        message_element.innerText = `ok: ${message}`;
    }

    function ok(message) {
        console.log(message);
        message_element.innerText = `ok: ${message}`;
    }

    function output_error(message) {
        console.log(message);
        message_element.innerText = `error: ${message}`;
    }

    let port = await tauri_invoke("backend");
    let address = `ws://localhost:${port}`;
    output(`attempting to connect to '${address}'`);

    let start = Date.now();
    let ws = new WebSocket(address);

    ws.onerror = async (error) => {
        let message = `failed to connect to server at '${address}' caused by an error. Click 'open' to open the dev-console for more information`;
        output_error(message);

        if (do_not_show_again) return;
        do_not_show_again = true;

        let open_dev = await tauri_ask(elapsed(start, Date.now(), message), {
            okLabel: "open",
            cancelLabel: "no",
            type: "error",
            title: "controller socket error"
        });
        if (open_dev) { tauri_invoke('open_devtools'); }
    }

    ws.onopen = () => {
        ok(`connected to ${address}. prompt will be replaced once the commander calls the clear method`);
    }

    ws.onclose = () => {
        output_error(`Connection to command center was closed. Attempting to reconnect in 1s`);
        setTimeout(() => main().then(), 1000);
    }

    ws.onmessage = async (data) => {
        let buffer = new Uint8Array(await data.data.arrayBuffer());
        let decoded = decoder.decode(0n, buffer)[0] as Message;
        let class_id = Number(decoded.class);
        if (decoded.kind.create)       create        (class_id, Number(decoded.kind.create.parent), decoded.kind.create.kind);
        if (decoded.kind.delete)       delete_element(class_id);
        if (decoded.kind.set_text)     set_text      (class_id, decoded.kind.set_text.text);
        if (decoded.kind.set_property) set_property  (class_id, decoded.kind.set_property.kind, decoded.kind.set_property.property, decoded.kind.set_property.value);
    }
}

