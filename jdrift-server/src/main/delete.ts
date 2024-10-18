export function delete_element(class_id: number, buffer_body: HTMLBodyElement) {
    buffer_body.querySelector(`.class-${class_id}`)?.remove();
}