import {PropertyKind} from "../main";

export function set_property(class_id: number, kind: PropertyKind, property: string, value: string) {
    let element = document.querySelector(`.class-${class_id}`) as any;
    if (!element) return;
    if (kind.style) { element.style[property] = value }
    else if (kind.attribute) { element.setAttribute(property, value) }
}