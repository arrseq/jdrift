export function set_text(class_id: number, buffer_body: HTMLBodyElement, text: string) {
    let element = buffer_body.querySelector(`.class-${class_id}`);
    if (!element) return;
    element.innerHTML = text;
}