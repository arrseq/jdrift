import {Decoder} from "xbinser/src/lib";

export interface Kind {
    division?:  {};
    span?:      {};
    paragraph?: {};
    button?:    {};
    header?:    {};
    canvas?:    {};
}

let mouse_button = "enum[primary[], scroll[], secondary[]]";

let encoder = new Decoder({
    class: "u32",
    kind: ["enum[",
        `mouse_click[button: ${mouse_button}, pressed: bool]`,
    "]"].join("")
});

export interface MouseButton {
    primary?: {};
    scroll?: {};
    secondary?: {};
}

export interface EventMessage {
    class: number,
    kind: {
        mouse_click?: { button: MouseButton, pressed: boolean }
    }
}

function send_event(ws: WebSocket, message: EventMessage) {
    ws.send(encoder.encode(message));
}

export function create(class_id: number, buffer_body: HTMLBodyElement, parent: number, kind: Kind, ws: WebSocket) {
    let element: HTMLElement | undefined;

    if (kind.division) { element = document.createElement("div"); }
    else if (kind.span) { element = document.createElement("span"); }
    else if (kind.paragraph) { element = document.createElement("p"); }
    else if (kind.button) { element = document.createElement("button"); }
    else if (kind.header) { element = document.createElement("h1"); }
    else if (kind.canvas) { element = document.createElement("canvas"); }

    if (element) {
        element.className = `class-${class_id}`;

        element.addEventListener("mousedown", () => {
            console.log("Down", class_id);

            send_event(ws, {
                class: class_id,
                kind: {
                    mouse_click: { button: { primary: {} }, pressed: true }
                }
            });
        });

        element.addEventListener("mouseup", () => {
            send_event(ws, {
                class: class_id,
                kind: {
                    mouse_click: { button: { primary: {} }, pressed: false }
                }
            });
        });

        buffer_body.querySelector(`.class-${parent}`)?.appendChild(element);
    }
}