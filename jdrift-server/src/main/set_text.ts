export function set_text(class_id: number, text: string) {
    let element = document.querySelector(`.class-${class_id}`);
    if (!element) return;
    element.innerHTML = text;
}