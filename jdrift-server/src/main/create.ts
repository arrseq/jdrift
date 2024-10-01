export interface Kind {
    division?:  {};
    span?:      {};
    paragraph?: {};
    button?:    {};
    header?:    {};
    canvas?:     {};
}

export function create(class_id: number, parent: number, kind: Kind) {
    let element: HTMLElement | undefined;

    if (kind.division) { element = document.createElement("div"); }
    else if (kind.span) { element = document.createElement("span"); }
    else if (kind.paragraph) { element = document.createElement("p"); }
    else if (kind.button) { element = document.createElement("button"); }
    else if (kind.header) { element = document.createElement("h1"); }
    else if (kind.canvas) { element = document.createElement("canvas"); }

    if (element) {
        element.className = `class-${class_id}`;
        document.querySelector(`.class-${parent}`)?.appendChild(element);
    }
}